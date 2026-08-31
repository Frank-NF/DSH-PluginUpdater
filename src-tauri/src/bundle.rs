//! Bundle 事务安装（V2 §3 状态机验证通过版）
//!
//! 事务机：PRECHECK → BACKUP → DOWNLOAD/INSTALL（逐插件）→ VERIFY → COMMIT；
//! 任一步失败立即 ROLLBACK。强制规则（V2 §3）：
//! 1. 失败即回滚，重试 = 用户重新发起（新事务）——本实现不做原地重试；
//! 2. 分阶段取消：BACKUP 前 cancel = 中止（CANCELLED，环境未动）；进入 DOWNLOAD 后 cancel = 触发 ROLLBACK；
//! 3. 取消仲裁：verify_ok 之后到达的 cancel 降级为 no-op；
//! 4. 回滚失败：保留备份 + 明确错误，绝不删除备份；
//! 5. 每事务一个 job（task_id + 取消令牌），进度事件按 task_id 路由。

use crate::error::{AppConfig, AppError, AppResult};
use crate::file_ops::PluginFileManager;
use crate::{catalog, version_probe, AppState};
use crate::github_proxy::GitHubProxyClient;
use serde::{Deserialize, Serialize};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::{Emitter, State};

const TIMEOUT: Duration = Duration::from_secs(20);
/// 官网组合包 API（与 catalog.rs 官网源同一 host）
const BUNDLES_URL: &str = "https://dsh.huilinsh.cn/api/bundles";
const BUNDLES_API_BASE: &str = "https://dsh.huilinsh.cn/api/bundles";
const BUNDLE_DETAIL_URL: &str = "https://dsh.huilinsh.cn/api/bundle";
/// 磁盘缓存目录（与 catalog.rs 同目录：%APPDATA%/dsh-plugin-updater/）
const CACHE_DIR_NAME: &str = "dsh-plugin-updater";

// ---------- 协议结构（与官网 API 返回对齐，camelCase） ----------

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundlePluginRef {
    #[serde(default)]
    pub plugin_ref: String,
    #[serde(default = "default_true")]
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleMcpServerDef {
    #[serde(default)]
    pub server_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub transport: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env_keys: Vec<String>,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleSkillDef {
    #[serde(default)]
    pub skill_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleDef {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub min_dsh_version: Option<String>,
    #[serde(default)]
    pub max_dsh_version: Option<String>,
    #[serde(default)]
    pub recommend_preset: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub plugins: Vec<BundlePluginRef>,
    #[serde(default)]
    pub mcp_servers: Vec<BundleMcpServerDef>,
    #[serde(default)]
    pub skills: Vec<BundleSkillDef>,
}

/// 整包预检的冲突条目（来自官网 plugin_conflicts 知识库）
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleConflictInfo {
    pub conflict_with: String,
    pub reason: Option<String>,
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundlePreviewItem {
    pub plugin_ref: String,
    pub required: bool,
    pub installed: bool,
    pub current_version: Option<String>,
    /// install=新装 / overwrite=覆盖已装 / skip=已最新
    pub action: String,
    /// 与当前已装插件集的已知冲突（空 = 无）
    #[serde(default)]
    pub conflicts: Vec<BundleConflictInfo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundlePreview {
    pub bundle: BundleDef,
    pub target_dir: String,
    pub items: Vec<BundlePreviewItem>,
    pub mcp_servers: Vec<BundleMcpServerDef>,
    pub skills: Vec<BundleSkillDef>,
    /// 整包预检结果（官网知识库 fail-open；不可达时 None）
    pub compat_all_compatible: Option<bool>,
    pub has_blocking_conflict: Option<bool>,
}

/// bundle_progress 事件载荷（stage: precheck/backup/download/install/verify/commit/rollback/cancelled）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleProgress {
    pub task_id: String,
    pub bundle_id: String,
    pub stage: String,
    pub percent: u8,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundlePluginResult {
    pub plugin_ref: String,
    pub status: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleInstallResult {
    pub task_id: String,
    pub bundle_id: String,
    /// committed | cancelled | rolled_back | failed（V2 §2 install.status 白名单）
    pub status: String,
    pub message: String,
    pub plugins: Vec<BundlePluginResult>,
}

/// 安装记录（JSON Lines 追加写，V2 §3 规则 6 审计）
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BundleInstallRecord<'a> {
    time: String,
    bundle_id: &'a str,
    version: &'a str,
    result: &'a str,
    plugins: &'a [BundlePluginResult],
}

// ---------- 内部计划 ----------

struct PluginPlan {
    npm: String,
    installed_before: bool,
    expected_version: Option<String>,
    skip: bool,
    backup_path: Option<String>,
    touched: bool,
    status: String,
}

impl PluginPlan {
    fn to_result(&self) -> BundlePluginResult {
        BundlePluginResult {
            plugin_ref: self.npm.clone(),
            status: self.status.clone(),
            detail: String::new(),
        }
    }
}

// ---------- 官网源与磁盘缓存 ----------

#[derive(Deserialize)]
struct BundlesIndexResponse {
    #[serde(default)]
    bundles: Vec<BundleDef>,
}

fn cache_file_path() -> Option<PathBuf> {
    // 测试/离线场景可用 DSH_BUNDLES_CACHE 覆盖缓存位置
    if let Ok(p) = std::env::var("DSH_BUNDLES_CACHE") {
        if !p.trim().is_empty() {
            return Some(PathBuf::from(p));
        }
    }
    let dir = dirs::config_dir()?;
    let cache_dir = dir.join(CACHE_DIR_NAME);
    std::fs::create_dir_all(&cache_dir).ok()?;
    Some(cache_dir.join("bundles.json"))
}

fn write_cache(list: &[BundleDef]) {
    if let Some(path) = cache_file_path() {
        if let Ok(json) = serde_json::to_string(list) {
            let _ = std::fs::write(path, json);
        }
    }
}

fn read_cache() -> Option<Vec<BundleDef>> {
    let path = cache_file_path()?;
    let raw = std::fs::read_to_string(path).ok()?;
    let list: Vec<BundleDef> = serde_json::from_str(&raw).ok()?;
    if list.is_empty() { None } else { Some(list) }
}

async fn fetch_bundles(client: &reqwest::Client) -> AppResult<Vec<BundleDef>> {
    let resp = client
        .get(BUNDLES_URL)
        .timeout(TIMEOUT)
        .send()
        .await?;
    if !resp.status().is_success() {
        return Err(AppError::Other(format!(
            "官网组合包索引 HTTP {}",
            resp.status()
        )));
    }
    let parsed: BundlesIndexResponse = resp.json().await?;
    Ok(parsed.bundles)
}

async fn fetch_bundle_detail(client: &reqwest::Client, id: &str) -> AppResult<Option<BundleDef>> {
    let url = format!("{}?id={}", BUNDLE_DETAIL_URL, urlencoding::encode(id));
    let resp = client.get(&url).timeout(TIMEOUT).send().await?;
    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Err(AppError::Other(format!("官网组合包详情 HTTP {}", resp.status())));
    }
    let bundle: BundleDef = resp.json().await?;
    Ok(Some(bundle))
}

/// 官网不可达时从磁盘缓存找单包
fn find_bundle_in_cache(id: &str) -> Option<BundleDef> {
    let list = read_cache()?;
    list.into_iter().find(|b| b.id == id)
}

// ---------- 公共小工具 ----------

fn resolve_target_dir(config: &AppConfig) -> String {
    let dir = config.plugin_directory.trim();
    if !dir.is_empty() && Path::new(dir).is_dir() {
        return dir.to_string();
    }
    // 兜底：默认 desktop profile（与 auto_scan 候选一致）
    if let Some(home) = dirs::home_dir() {
        let p = home.join(".dsh").join("profiles").join("desktop");
        if p.is_dir() {
            return p.to_string_lossy().to_string();
        }
    }
    dir.to_string()
}

fn installed_plugin_dir(target_dir: &str, plugin_ref: &str) -> PathBuf {
    Path::new(target_dir).join("node_modules").join(plugin_ref)
}

fn read_installed_version(target_dir: &str, plugin_ref: &str) -> Option<String> {
    let pkg = installed_plugin_dir(target_dir, plugin_ref).join("package.json");
    let content = std::fs::read_to_string(pkg).ok()?;
    let v: serde_json::Value = serde_json::from_str(&content).ok()?;
    v.get("version")
        .and_then(|x| x.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// npm install 核心：与市场单插件安装（install_plugin）共用同一条安装链路。
/// Windows 经 cmd /c 调 npm（PATHEXT 解析 npm.cmd，绕过 PowerShell 执行策略），超时 300s。
pub(crate) async fn npm_install_into(npm_name: &str, target_dir: &str, registry: &str) -> Result<(), String> {
    let mut cmd = if cfg!(windows) {
        let mut c = tokio::process::Command::new("cmd");
        c.arg("/c").arg("npm");
        c
    } else {
        tokio::process::Command::new("npm")
    };
    let mut npm_args: Vec<String> = vec![
        "install".into(),
        npm_name.to_string(),
        "--prefix".into(),
        target_dir.to_string(),
        "--no-audit".into(),
        "--no-fund".into(),
        // DSH 插件族 peer 依赖互相冲突，npm7+ 默认严格解析会 ERESOLVE
        "--legacy-peer-deps".into(),
        "--loglevel".into(),
        "error".into(),
    ];
    let registry = registry.trim();
    if !registry.is_empty() {
        npm_args.push("--registry".into());
        npm_args.push(registry.to_string());
    }
    cmd.args(&npm_args);
    cmd.current_dir(target_dir);

    let output = tokio::time::timeout(Duration::from_secs(300), cmd.output())
        .await
        .map_err(|_| "npm install 超时（5 分钟）".to_string())
        .and_then(|r| {
            r.map_err(|e| {
                format!(
                    "启动 npm 失败: {}（请确认已安装 Node.js/npm 并在 PATH 中）",
                    e
                )
            })
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !output.status.success() {
        let detail = if stderr.trim().is_empty() { stdout } else { stderr };
        let d = detail.trim().to_string();
        let detail = if d.chars().count() > 600 {
            d.chars().skip(d.chars().count() - 600).collect::<String>()
        } else {
            d
        };
        return Err(format!("npm install 失败: {}", detail));
    }
    Ok(())
}

fn append_install_record(
    plugin_directory: &str,
    bundle_id: &str,
    version: &str,
    result: &str,
    plugins: &[BundlePluginResult],
) -> AppResult<()> {
    let dir = Path::new(plugin_directory).join(".updater_backups");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("bundle_installs.json");
    let record = BundleInstallRecord {
        time: chrono::Local::now().to_rfc3339(),
        bundle_id,
        version,
        result,
        plugins,
    };
    let mut line = serde_json::to_string(&record)?;
    line.push('\n');
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&path)?;
    f.write_all(line.as_bytes())?;
    Ok(())
}

/// 安装成功后 MCP 层：把 bundle 的 mcp_servers 合并写入 ~/.dsh/dsh-mcp.json（全局）。
/// 文件存在则保留既有条目，只补缺失的 server_id；env 只写键名 + 空字符串值（V2 §2）。
/// 失败由调用方仅 log 告警，不影响安装事务结果。
fn merge_mcp_servers(servers: &[BundleMcpServerDef]) -> AppResult<()> {
    if servers.is_empty() {
        return Ok(());
    }
    // 测试/离线场景可用 DSH_MCP_PATH 覆盖目标文件位置（默认 ~/.dsh/dsh-mcp.json）
    let path = match std::env::var("DSH_MCP_PATH") {
        Ok(p) if !p.trim().is_empty() => PathBuf::from(p),
        _ => {
            let home = dirs::home_dir().ok_or_else(|| AppError::Other("无法定位用户主目录".into()))?;
            home.join(".dsh").join("dsh-mcp.json")
        }
    };
    let mut root: serde_json::Value = if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let v: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| AppError::Other(format!("dsh-mcp.json 解析失败: {}", e)))?;
        if !v.is_object() {
            return Err(AppError::Other("dsh-mcp.json 顶层不是对象".into()));
        }
        v
    } else {
        serde_json::json!({ "mcpServers": {} })
    };

    let existing = root.get("mcpServers").cloned().unwrap_or_else(|| serde_json::json!({}));
    let mut obj = match existing {
        serde_json::Value::Object(o) => o,
        _ => serde_json::Map::new(),
    };
    let mut added = 0usize;
    for s in servers {
        let sid = s.server_id.trim();
        if sid.is_empty() || obj.contains_key(sid) {
            continue; // 既有条目零丢失：只补缺失 server_id
        }
        // 枚举白名单（V2 §2）：transport 仅 stdio / streamable-http
        let transport = if s.transport.trim() == "streamable-http" { "streamable-http" } else { "stdio" };
        let mut entry = serde_json::Map::new();
        entry.insert("name".into(), serde_json::Value::String(s.name.clone()));
        entry.insert("transport".into(), serde_json::Value::String(transport.into()));
        if transport == "stdio" {
            entry.insert("command".into(), serde_json::Value::String(s.command.clone()));
            entry.insert(
                "args".into(),
                serde_json::Value::Array(
                    s.args.iter().map(|a| serde_json::Value::String(a.clone())).collect()
                ),
            );
        }
        let mut env = serde_json::Map::new();
        for k in &s.env_keys {
            env.insert(k.clone(), serde_json::Value::String(String::new()));
        }
        entry.insert("env".into(), serde_json::Value::Object(env));
        entry.insert("description".into(), serde_json::Value::String(s.description.clone()));
        obj.insert(sid.to_string(), serde_json::Value::Object(entry));
        added += 1;
    }
    root["mcpServers"] = serde_json::Value::Object(obj);
    if added > 0 {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // 原子写：先临时文件再改名
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, serde_json::to_string_pretty(&root)?)?;
        std::fs::rename(&tmp, &path)?;
        log::info!("[bundle] dsh-mcp.json 已合并 {} 个 MCP 服务", added);
    }
    Ok(())
}

// ---------- Tauri 命令 ----------

/// 官网组合包全量索引：成功刷新磁盘缓存，失败读缓存，再失败空列表
#[tauri::command]
pub async fn list_bundles(state: State<'_, AppState>) -> AppResult<Vec<BundleDef>> {
    let config = state
        .config
        .lock()
        .map_err(|_| AppError::Other("配置锁不可用".into()))?
        .clone();
    let proxy = GitHubProxyClient::new(&config.proxy_base_url, None);
    match fetch_bundles(proxy.http_client()).await {
        Ok(list) => {
            if list.is_empty() {
                // 空索引回落缓存，避免官网异常清空本地可见集合
                if let Some(cached) = read_cache() {
                    return Ok(cached);
                }
                return Ok(Vec::new());
            }
            // 成功时刷新磁盘缓存
            write_cache(&list);
            Ok(list)
        }
        Err(e) => {
            log::warn!("[bundle] 官网组合包索引拉取失败，降级磁盘缓存: {}", e);
            Ok(read_cache().unwrap_or_default())
        }
    }
}

/// 组合包逐项预检：插件已装/未装、版本、将执行动作（install/overwrite/skip）
#[tauri::command]
pub async fn preview_bundle(id: String, state: State<'_, AppState>) -> AppResult<BundlePreview> {
    let config = state
        .config
        .lock()
        .map_err(|_| AppError::Other("配置锁不可用".into()))?
        .clone();
    let proxy = GitHubProxyClient::new(&config.proxy_base_url, None);
    let bundle = match fetch_bundle_detail(proxy.http_client(), &id).await {
        Ok(Some(b)) => b,
        Ok(None) => return Err(AppError::Other(format!("组合包不存在: {}", id))),
        Err(e) => {
            log::warn!("[bundle] 详情拉取失败，尝试磁盘缓存: {}", e);
            match find_bundle_in_cache(&id) {
                Some(b) => b,
                None => {
                    return Err(AppError::Other(format!(
                        "组合包不可得（官网与缓存均失败）: {}",
                        id
                    )));
                }
            }
        }
    };
    let target_dir = resolve_target_dir(&config);
    let mut items = Vec::new();
    for pref in &bundle.plugins {
        let npm = pref.plugin_ref.trim().to_string();
        if npm.is_empty() {
            continue;
        }
        let installed = installed_plugin_dir(&target_dir, &npm)
            .join("package.json")
            .is_file();
        let current_version = read_installed_version(&target_dir, &npm);
        let expected = catalog::npm_latest_version(proxy.http_client(), &npm)
            .await
            .ok();
        let action = if !installed {
            "install"
        } else if current_version.is_some() && expected.is_some() && current_version == expected {
            "skip"
        } else {
            "overwrite"
        };
        items.push(BundlePreviewItem {
            plugin_ref: npm,
            required: pref.required,
            installed,
            current_version,
            action: action.to_string(),
            conflicts: Vec::new(),
        });
    }

    // 整包预检（V2 §8 P1）：把已装插件集传给官网知识库，一次取回全部冲突。
    // fail-open：官网不可达/解析失败时不阻塞预览，仅无冲突信息。
    let installed_list: Vec<&str> = items
        .iter()
        .filter(|i| i.installed)
        .map(|i| i.plugin_ref.as_str())
        .collect();
    let mut compat_all_compatible: Option<bool> = None;
    let mut has_blocking_conflict: Option<bool> = None;
    if !installed_list.is_empty() {
        let url = format!(
            "{}/compat/check?id={}&installed={}",
            BUNDLES_API_BASE,
            urlencoding::encode(&id),
            urlencoding::encode(&installed_list.join(",")),
        );
        match proxy
            .http_client()
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                #[derive(serde::Deserialize)]
                struct BundleCompatResp {
                    #[serde(default)]
                    items: Vec<BundleCompatItem>,
                    #[serde(default)]
                    all_compatible: Option<bool>,
                    #[serde(default)]
                    has_blocking_conflict: Option<bool>,
                }
                #[derive(serde::Deserialize)]
                struct BundleCompatItem {
                    plugin_ref: String,
                    #[serde(default)]
                    compatible: Option<bool>,
                    #[serde(default)]
                    conflicts: Vec<BundleConflictInfo>,
                }
                match resp.json::<BundleCompatResp>().await {
                    Ok(parsed) => {
                        for item in &mut items {
                            if let Some(ci) = parsed.items.iter().find(|c| c.plugin_ref == item.plugin_ref) {
                                item.conflicts = ci.conflicts.clone();
                            }
                        }
                        compat_all_compatible = parsed.all_compatible;
                        has_blocking_conflict = parsed.has_blocking_conflict;
                    }
                    Err(e) => eprintln!("[bundle] 整包预检响应解析失败，跳过: {}", e),
                }
            }
            Ok(resp) => eprintln!("[bundle] 整包预检 HTTP {}，跳过", resp.status()),
            Err(e) => eprintln!("[bundle] 整包预检请求失败，跳过: {}", e),
        }
    }

    let mcp_servers = bundle.mcp_servers.clone();
    let skills = bundle.skills.clone();
    Ok(BundlePreview {
        bundle,
        target_dir,
        items,
        mcp_servers,
        skills,
        compat_all_compatible,
        has_blocking_conflict,
    })
}

// ---------- 事务机（V2 §3） ----------

fn truncate_str(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let head: String = s.chars().take(max_chars).collect();
        format!("{}…", head)
    }
}

/// 对本次已动过的插件回滚：有备份的 restore_backup，新装的移除目录。
/// 返回失败明细；回滚失败绝不删除备份（V2 §3 规则 4）。
fn rollback_touched(
    target_dir: &str,
    plan: &[PluginPlan],
    file_manager: &PluginFileManager,
) -> Vec<String> {
    let mut failures = Vec::new();
    for p in plan {
        if !p.touched {
            continue;
        }
        let install_dir = installed_plugin_dir(target_dir, &p.npm);
        if let Some(backup) = &p.backup_path {
            if let Err(e) = file_manager.restore_backup(backup, &install_dir.to_string_lossy()) {
                failures.push(format!(
                    "{} 回滚失败（备份已保留: {}）: {}",
                    p.npm, backup, e
                ));
            }
        } else if let Err(e) = std::fs::remove_dir_all(&install_dir) {
            if e.kind() != std::io::ErrorKind::NotFound {
                failures.push(format!("{} 移除新装目录失败: {}", p.npm, e));
            }
        }
    }
    failures
}

struct RollbackOutcome {
    status: &'static str,
    message: String,
}

fn finish_after_rollback(failures: Vec<String>, cancelled: bool, reason: &str) -> RollbackOutcome {
    if failures.is_empty() {
        if cancelled {
            RollbackOutcome { status: "cancelled", message: format!("{}；已回滚到安装前状态", reason) }
        } else {
            RollbackOutcome { status: "rolled_back", message: format!("{}；已回滚到安装前状态", reason) }
        }
    } else {
        RollbackOutcome {
            status: "failed",
            message: format!(
                "{}；回滚未完成（备份已保留在 .updater_backups，请勿删除，可手动恢复）: {}",
                reason,
                failures.join("；")
            ),
        }
    }
}

/// 安装入口：生成任务 id + 取消令牌（is_cancelled 按 task_id 查询），
/// 结束后清理令牌。重试语义 = 用户重新发起（新事务，V2 §3 规则 1）。
#[tauri::command]
pub async fn install_bundle(
    id: String,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> AppResult<BundleInstallResult> {
    let task_id = format!("bundle-{}", chrono::Local::now().format("%Y%m%d%H%M%S%3f"));
    let cancel = Arc::new(AtomicBool::new(false));
    {
        let mut map = state
            .bundle_cancels
            .lock()
            .map_err(|_| AppError::Other("取消令牌表不可用".into()))?;
        map.insert(task_id.clone(), cancel.clone());
    }
    let config = state
        .config
        .lock()
        .map_err(|_| AppError::Other("配置锁不可用".into()))?
        .clone();
    let emit = |stage: &str, percent: u8, message: String| {
        let _ = window.emit(
            "bundle_progress",
            BundleProgress {
                task_id: task_id.clone(),
                bundle_id: id.clone(),
                stage: stage.to_string(),
                percent,
                message,
            },
        );
    };
    let result = run_transaction(&id, &task_id, &config, &cancel, emit).await;
    {
        let mut map = state
            .bundle_cancels
            .lock()
            .map_err(|_| AppError::Other("取消令牌表不可用".into()))?;
        map.remove(&task_id);
    }
    result
}

/// 取消令牌查询命令（前端在每阶段边界展示取消按钮的依据）
#[tauri::command]
pub fn is_cancelled(task_id: String, state: State<'_, AppState>) -> bool {
    state
        .bundle_cancels
        .lock()
        .map(|m| {
            m.get(&task_id)
                .map(|f| f.load(Ordering::SeqCst))
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

/// 请求取消某个安装事务（令牌置位，事务在下一步边界响应）
#[tauri::command]
pub fn cancel_bundle_install(task_id: String, state: State<'_, AppState>) -> bool {
    state
        .bundle_cancels
        .lock()
        .map(|m| {
            if let Some(f) = m.get(&task_id) {
                f.store(true, Ordering::SeqCst);
                true
            } else {
                false
            }
        })
        .unwrap_or(false)
}

/// 事务主体：PRECHECK → BACKUP → DOWNLOAD/INSTALL → VERIFY → COMMIT（任一步失败即 ROLLBACK）
async fn run_transaction(
    id: &str,
    task_id: &str,
    config: &AppConfig,
    cancel: &AtomicBool,
    emit: impl Fn(&str, u8, String),
) -> AppResult<BundleInstallResult> {
    // 进度上报由调用方注入：Tauri 命令走 bundle_progress 事件，E2E 测试收集事件向量
    let proxy = GitHubProxyClient::new(&config.proxy_base_url, None);

    // ---------- PRECHECK ----------
    emit("precheck", 5, "正在预检组合包…".into());
    let bundle = match fetch_bundle_detail(proxy.http_client(), id).await {
        Ok(Some(b)) => b,
        Ok(None) => {
            // 官网明确 404：缓存=最后一次已知镜像，仍可兜底（与 list_bundles 缓存语义一致）
            log::warn!("[bundle] 官网无此组合包（404），尝试磁盘缓存: {}", id);
            match find_bundle_in_cache(id) {
                Some(b) => b,
                None => return Err(AppError::Other(format!("组合包不存在: {}", id))),
            }
        }
        Err(e) => {
            log::warn!("[bundle] 详情拉取失败，尝试磁盘缓存: {}", e);
            match find_bundle_in_cache(id) {
                Some(b) => b,
                None => {
                    return Err(AppError::Other(format!(
                        "组合包不可得（官网与缓存均失败）: {}",
                        id
                    )));
                }
            }
        }
    };
    if bundle.plugins.is_empty() {
        return Err(AppError::Other("组合包不含任何插件，无需安装".into()));
    }
    let target_dir = resolve_target_dir(config);
    if target_dir.is_empty() || !Path::new(&target_dir).join("package.json").is_file() {
        return Err(AppError::Other(format!(
            "安装目标不是 DSH profile 目录（缺少 package.json）: {}",
            target_dir
        )));
    }
    // 版本区间校验（P0 从简：V2 §2 允许 min="*" 通配直过；锚点值定稿后启用双端校验）
    if let Some(minv) = bundle
        .min_dsh_version
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty() && *s != "*")
    {
        match version_probe::read_dsh_version(&config.plugin_directory) {
            Some(cur) => match (
                semver::Version::parse(&cur),
                semver::Version::parse(minv),
            ) {
                (Ok(c), Ok(m)) if c < m => {
                    return Err(AppError::Other(format!(
                        "DSH 版本过低：当前 v{}，组合包要求 >= {}",
                        cur, minv
                    )));
                }
                _ => {}
            },
            None => log::warn!("[bundle] 无法探测 DSH 运行时版本，跳过 min_dsh_version 校验"),
        }
    }
    // 取消（BACKUP 前 cancel=中止，环境未动）——V2 §3 规则 2
    if cancel.load(Ordering::SeqCst) {
        emit("cancelled", 100, "安装已取消（未做任何改动）".into());
        let plugins: Vec<BundlePluginResult> = Vec::new();
        let _ = append_install_record(
            &config.plugin_directory,
            id,
            bundle.version.as_deref().unwrap_or(""),
            "cancelled",
            &plugins,
        );
        return Ok(BundleInstallResult {
            task_id: task_id.to_string(),
            bundle_id: id.to_string(),
            status: "cancelled".into(),
            message: "安装已取消（未做任何改动）".into(),
            plugins,
        });
    }

    // 逐插件预检计划（期望版本 = npm latest，尽力而为；不可达时 fail-open）
    let mut plan: Vec<PluginPlan> = Vec::new();
    for pref in &bundle.plugins {
        let npm = pref.plugin_ref.trim().to_string();
        if npm.is_empty() {
            continue;
        }
        let installed_before = installed_plugin_dir(&target_dir, &npm)
            .join("package.json")
            .is_file();
        let current = read_installed_version(&target_dir, &npm);
        let expected_version = catalog::npm_latest_version(proxy.http_client(), &npm)
            .await
            .ok();
        let skip = installed_before
            && current.is_some()
            && expected_version.is_some()
            && current == expected_version;
        plan.push(PluginPlan {
            npm,
            installed_before,
            expected_version,
            skip,
            backup_path: None,
            touched: false,
            status: if skip { "skipped".into() } else { "pending".into() },
        });
    }
    if plan.is_empty() {
        return Err(AppError::Other("组合包内没有有效的插件引用".into()));
    }
    let file_manager = PluginFileManager::new(&config.plugin_directory);

    // ---------- BACKUP（覆盖安装的插件逐个走 backup_plugin；fail→FAILED，环境未动） ----------
    let backup_targets: Vec<(usize, String)> = plan
        .iter()
        .enumerate()
        .filter(|(_, p)| p.installed_before && !p.skip)
        .map(|(i, p)| {
            let path = installed_plugin_dir(&target_dir, &p.npm)
                .to_string_lossy()
                .to_string();
            (i, path)
        })
        .collect();
    let total_backup = backup_targets.len();
    for (k, (idx, path)) in backup_targets.iter().enumerate() {
        if cancel.load(Ordering::SeqCst) {
            // BACKUP 前/中取消 = 中止（环境未动；已生成的备份文件保留）
            emit("cancelled", 100, "安装已取消（未做任何改动）".into());
            let plugins: Vec<BundlePluginResult> = plan.iter().map(|p| p.to_result()).collect();
            let _ = append_install_record(
                &config.plugin_directory,
                id,
                bundle.version.as_deref().unwrap_or(""),
                "cancelled",
                &plugins,
            );
            return Ok(BundleInstallResult {
                task_id: task_id.to_string(),
                bundle_id: id.to_string(),
                status: "cancelled".into(),
                message: "安装已取消（未做任何改动）".into(),
                plugins,
            });
        }
        let name = plan[*idx].npm.clone();
        let pct = (10 + 8 * (k + 1) / total_backup.max(1)).min(18) as u8;
        emit("backup", pct, format!("正在备份 {}…", name));
        // 备份名不能含路径分隔符（scoped 包名如 @scope/name）
        let safe_id = name.replace(['/', '\\'], "_");
        match file_manager.backup_plugin(path, &safe_id) {
            Ok(backup_path) => plan[*idx].backup_path = Some(backup_path),
            Err(e) => {
                // BACKUP fail → FAILED（环境未动，无需回滚）
                let msg = format!("备份失败（{}）: {}", name, e);
                emit("failed", 100, msg.clone());
                for p in plan.iter_mut() {
                    if p.status == "pending" {
                        p.status = "failed".into();
                    }
                }
                let plugins: Vec<BundlePluginResult> = plan.iter().map(|p| p.to_result()).collect();
                let _ = append_install_record(
                    &config.plugin_directory,
                    id,
                    bundle.version.as_deref().unwrap_or(""),
                    "failed",
                    &plugins,
                );
                return Err(AppError::Other(msg));
            }
        }
    }

    // ---------- DOWNLOAD / INSTALL（逐插件；npm install 融合下载+安装） ----------
    // 进入本阶段后 cancel = 触发 ROLLBACK（V2 §3 规则 2）；
    // 安装失败不原地重试（修正版 A11）——重试 = 用户重新发起新事务（V2 §3 规则 1）。
    let registry = config.install_registry.trim().to_string();
    let install_list: Vec<usize> = (0..plan.len()).filter(|&i| !plan[i].skip).collect();
    let total_install = install_list.len();
    for (k, &i) in install_list.iter().enumerate() {
        if cancel.load(Ordering::SeqCst) {
            let reason = "检测到取消请求".to_string();
            emit("rollback", 90, format!("{}，正在回滚…", reason));
            let failures = rollback_touched(&target_dir, &plan, &file_manager);
            let out = finish_after_rollback(failures, true, &reason);
            for p in plan.iter_mut() {
                if p.status == "pending" {
                    p.status = "skipped".into();
                }
            }
            let plugins: Vec<BundlePluginResult> = plan.iter().map(|p| p.to_result()).collect();
            let _ = append_install_record(
                &config.plugin_directory,
                id,
                bundle.version.as_deref().unwrap_or(""),
                out.status,
                &plugins,
            );
            let final_stage = if out.status == "failed" { "failed" } else { "rollback" };
            emit(final_stage, 100, out.message.clone());
            if out.status == "failed" {
                return Err(AppError::Other(out.message));
            }
            return Ok(BundleInstallResult {
                task_id: task_id.to_string(),
                bundle_id: id.to_string(),
                status: out.status.into(),
                message: out.message,
                plugins,
            });
        }
        let npm = plan[i].npm.clone();
        let dl_pct = (20 + 60 * k / total_install.max(1)).min(85) as u8;
        emit("download", dl_pct, format!("正在下载/安装 {}…", npm));
        if let Err(e) = npm_install_into(&npm, &target_dir, &registry).await {
            // INSTALL apply_fail → ROLLBACK（不原地重试）
            let reason = format!("{} 安装失败: {}", npm, truncate_str(&e, 200));
            plan[i].status = "failed".into();
            emit("rollback", 90, format!("{}，正在回滚…", reason));
            let failures = rollback_touched(&target_dir, &plan, &file_manager);
            let out = finish_after_rollback(failures, false, &reason);
            for p in plan.iter_mut() {
                if p.status == "pending" {
                    p.status = "skipped".into();
                }
            }
            let plugins: Vec<BundlePluginResult> = plan.iter().map(|p| p.to_result()).collect();
            let _ = append_install_record(
                &config.plugin_directory,
                id,
                bundle.version.as_deref().unwrap_or(""),
                out.status,
                &plugins,
            );
            let final_stage = if out.status == "failed" { "failed" } else { "rollback" };
            emit(final_stage, 100, out.message.clone());
            return Err(AppError::Other(out.message));
        }
        plan[i].touched = true;
        plan[i].status = "ok".into();
        let ins_pct = (20 + 60 * (k + 1) / total_install.max(1)).min(88) as u8;
        emit("install", ins_pct, format!("{} 安装完成", npm));
    }

    // ---------- VERIFY（P0：安装目录存在且 package.json 版本与预期一致；
    // SHA256 与官网核对链路 P1 再接，见 V2 §5.3） ----------
    emit("verify", 90, "正在校验安装结果…".into());
    let mut verify_fail: Option<String> = None;
    for p in plan.iter_mut() {
        if p.skip || !p.touched {
            continue;
        }
        if !installed_plugin_dir(&target_dir, &p.npm)
            .join("package.json")
            .is_file()
        {
            verify_fail = Some(format!("{} 安装目录缺失", p.npm));
            break;
        }
        if let Some(exp) = p.expected_version.clone() {
            match read_installed_version(&target_dir, &p.npm) {
                Some(actual) if actual == exp => {}
                Some(actual) => {
                    verify_fail = Some(format!("{} 版本不符：期望 {}，实际 {}", p.npm, exp, actual));
                    break;
                }
                None => {
                    verify_fail = Some(format!("{} 安装后 package.json 无版本号", p.npm));
                    break;
                }
            }
        }
    }
    if let Some(reason) = verify_fail {
        emit("rollback", 90, format!("{}，正在回滚…", reason));
        let failures = rollback_touched(&target_dir, &plan, &file_manager);
        let out = finish_after_rollback(failures, false, &reason);
        for p in plan.iter_mut() {
            if p.status == "pending" {
                p.status = "skipped".into();
            }
        }
        let plugins: Vec<BundlePluginResult> = plan.iter().map(|p| p.to_result()).collect();
        let _ = append_install_record(
            &config.plugin_directory,
            id,
            bundle.version.as_deref().unwrap_or(""),
            out.status,
            &plugins,
        );
        let final_stage = if out.status == "failed" { "failed" } else { "rollback" };
        emit(final_stage, 100, out.message.clone());
        return Err(AppError::Other(out.message));
    }

    // ---------- COMMIT（verify_ok 之后到达的 cancel 降级为 no-op，V2 §3 规则 3） ----------
    emit("commit", 96, "正在提交安装记录…".into());
    for p in plan.iter_mut() {
        if p.status == "pending" {
            p.status = "skipped".into();
        }
    }
    let plugins: Vec<BundlePluginResult> = plan.iter().map(|p| p.to_result()).collect();
    if let Err(e) = append_install_record(
        &config.plugin_directory,
        id,
        bundle.version.as_deref().unwrap_or(""),
        "committed",
        &plugins,
    ) {
        // commit_fail → ROLLBACK（V2 §3）
        let reason = format!("安装记录写入失败: {}", e);
        emit("rollback", 90, format!("{}，正在回滚…", reason));
        let failures = rollback_touched(&target_dir, &plan, &file_manager);
        let out = finish_after_rollback(failures, false, &reason);
        let final_stage = if out.status == "failed" { "failed" } else { "rollback" };
        emit(final_stage, 100, out.message.clone());
        return Err(AppError::Other(out.message));
    }

    // MCP 层：合并写入全局 dsh-mcp.json；失败仅告警，不影响安装事务结果
    if let Err(e) = merge_mcp_servers(&bundle.mcp_servers) {
        log::warn!("[bundle] dsh-mcp.json 合并失败（不影响安装结果）: {}", e);
    }

    emit(
        "commit",
        100,
        format!("组合包「{}」安装完成（{} 个插件）", bundle.name, total_install),
    );
    Ok(BundleInstallResult {
        task_id: task_id.to_string(),
        bundle_id: id.to_string(),
        status: "committed".into(),
        message: format!("已安装组合包「{}」（{} 个插件）", bundle.name, total_install),
        plugins,
    })
}

#[cfg(test)]
mod e2e_tests {
    //! 真机端到端验证（真实 npm 安装、真实文件系统、真实备份/回滚/MCP 合并）。
    //! DSH_BUNDLES_CACHE / DSH_MCP_PATH 两个 env 覆盖点保证测试零污染真实用户状态；
    //! env 为进程全局，用 SERIAL 串行化各用例。
    use super::*;
    use std::sync::Mutex as StdMutex;

    static SERIAL: StdMutex<()> = StdMutex::new(());

    type EventLog = Arc<StdMutex<Vec<(String, u8, String)>>>;

    fn temp_base(tag: &str) -> PathBuf {
        let base = std::env::temp_dir().join(format!("dsh-e2e-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).expect("create temp base");
        base
    }

    fn make_profile(base: &Path) -> PathBuf {
        let profile = base.join("profile");
        std::fs::create_dir_all(&profile).expect("create profile");
        std::fs::write(profile.join("package.json"), "{}\
").expect("write package.json");
        profile
    }

    fn seed_cache(base: &Path, tag: &str, plugins_json: &str) -> PathBuf {
        let cache = base.join("bundles.json");
        let bundle = format!(
            "[{{\"id\":\"e2e-{tag}\",\"name\":\"E2E {tag} 包\",\"description\":\"真机端到端\",\"tags\":[\"e2e\"],\"mode\":\"preset\",\"minDshVersion\":\"*\",\"version\":\"1.0.0\",\"plugins\":{plugins_json},\"mcpServers\":[],\"skills\":[]}}]",
            tag = tag,
            plugins_json = plugins_json,
        );
        std::fs::write(&cache, bundle).expect("seed cache");
        cache
    }

    fn base_config(profile: &Path) -> AppConfig {
        let mut config = AppConfig::default();
        config.plugin_directory = profile.to_string_lossy().to_string();
        config
    }

    fn event_collector() -> (EventLog, impl Fn(&str, u8, String)) {
        let log: EventLog = Arc::new(StdMutex::new(Vec::new()));
        let sink = log.clone();
        let emit = move |stage: &str, percent: u8, message: String| {
            if let Ok(mut v) = sink.lock() {
                v.push((stage.to_string(), percent, message));
            }
        };
        (log, emit)
    }

    fn record_text(profile: &Path) -> Option<String> {
        let path = profile.join(".updater_backups").join("bundle_installs.json");
        std::fs::read_to_string(path).ok()
    }

    /// 提交路径：预检 → 下载/安装（真实 npm）→ 校验 → 提交，node_modules 落盘且版本与预期一致
    #[tokio::test]
    async fn e2e_commit_installs_and_verifies() {
        let _g = SERIAL.lock().expect("serial");
        let base = temp_base("commit");
        let profile = make_profile(&base);
        let cache = seed_cache(&base, "commit", "[{\"pluginRef\":\"left-pad\",\"required\":true},{\"pluginRef\":\"ms\",\"required\":true}]");
        std::env::set_var("DSH_BUNDLES_CACHE", &cache);
        let config = base_config(&profile);
        let (log, emit) = event_collector();
        let result = run_transaction(
            "e2e-commit",
            "task-e2e-commit",
            &config,
            &AtomicBool::new(false),
            emit,
        )
        .await;
        match &result {
            Ok(r) => assert_eq!(r.status, "committed", "应为 committed：{}", r.message),
            Err(e) => panic!(
                "事务不应失败：{}（plugins 状态见 bundle_installs.json：{:?})",
                e,
                record_text(&profile),
            ),
        }
        let left_pad = profile.join("node_modules").join("left-pad").join("package.json");
        let ms = profile.join("node_modules").join("ms").join("package.json");
        assert!(left_pad.exists(), "left-pad 未落盘");
        assert!(ms.exists(), "ms 未落盘");
        // 校验阶段要求版本与 npm latest 一致，committed 即代表校验通过
        let record = record_text(&profile).expect("安装记录缺失");
        assert!(record.contains("\"result\":\"committed\""), "记录结果错误: {}", record);
        let stages: Vec<String> = log
            .lock()
            .expect("log")
            .iter()
            .map(|(s, _, _)| s.clone())
            .collect();
        for stage in ["precheck", "download", "install", "verify", "commit"] {
            assert!(stages.iter().any(|s| s == stage), "缺少阶段事件: {}（全部: {:?}）", stage, stages);
        }
        std::env::remove_var("DSH_BUNDLES_CACHE");
        let _ = std::fs::remove_dir_all(&base);
    }

    /// 回滚路径：npm 安装失败（不可达 registry）→ ROLLBACK，目录清理 + 记录 rolled_back
    #[tokio::test]
    async fn e2e_install_failure_rolls_back() {
        let _g = SERIAL.lock().expect("serial");
        let base = temp_base("rollback");
        let profile = make_profile(&base);
        let cache = seed_cache(&base, "rollback", "[{\"pluginRef\":\"ms\",\"required\":true}]");
        std::env::set_var("DSH_BUNDLES_CACHE", &cache);
        let mut config = base_config(&profile);
        config.install_registry = "http://127.0.0.1:9".into();
        let (log, emit) = event_collector();
        let result = run_transaction(
            "e2e-rollback",
            "task-e2e-rollback",
            &config,
            &AtomicBool::new(false),
            emit,
        )
        .await;
        assert!(result.is_err(), "registry 不可达时事务应失败");
        let msg = result.err().expect("err").to_string();
        assert!(msg.contains("已回滚"), "失败信息应说明已回滚: {}", msg);
        let ms_dir = profile.join("node_modules").join("ms");
        assert!(!ms_dir.exists(), "回滚后不应残留新装目录");
        let record = record_text(&profile).expect("安装记录缺失");
        assert!(record.contains("\"result\":\"rolled_back\""), "记录结果错误: {}", record);
        let stages: Vec<String> = log
            .lock()
            .expect("log")
            .iter()
            .map(|(s, _, _)| s.clone())
            .collect();
        assert!(stages.iter().any(|s| s == "rollback"), "缺少回滚事件: {:?}", stages);
        std::env::remove_var("DSH_BUNDLES_CACHE");
        let _ = std::fs::remove_dir_all(&base);
    }

    /// 取消路径：BACKUP 前置取消 = 中止（V2 §3 规则 2），零改动 + 记录 cancelled
    #[tokio::test]
    async fn e2e_cancel_before_backup_aborts() {
        let _g = SERIAL.lock().expect("serial");
        let base = temp_base("cancel");
        let profile = make_profile(&base);
        let cache = seed_cache(&base, "cancel", "[{\"pluginRef\":\"ms\",\"required\":true}]");
        std::env::set_var("DSH_BUNDLES_CACHE", &cache);
        let config = base_config(&profile);
        let (log, emit) = event_collector();
        let result = run_transaction(
            "e2e-cancel",
            "task-e2e-cancel",
            &config,
            &AtomicBool::new(true),
            emit,
        )
        .await;
        match &result {
            Ok(r) => assert_eq!(r.status, "cancelled", "应 cancelled：{}", r.message),
            Err(e) => panic!("前置取消不应报错: {}", e),
        }
        assert!(!profile.join("node_modules").exists(), "取消后不应有安装痕迹");
        let record = record_text(&profile).expect("安装记录缺失");
        assert!(record.contains("\"result\":\"cancelled\""), "记录结果错误: {}", record);
        let stages: Vec<String> = log
            .lock()
            .expect("log")
            .iter()
            .map(|(s, _, _)| s.clone())
            .collect();
        assert!(stages.iter().any(|s| s == "cancelled"), "缺少取消事件: {:?}", stages);
        std::env::remove_var("DSH_BUNDLES_CACHE");
        let _ = std::fs::remove_dir_all(&base);
    }

    /// MCP 合并：既有条目零丢失、缺失 server_id 补齐、env 只写键名空值、幂等
    #[tokio::test]
    async fn mcp_merge_preserves_existing_entries() {
        let _g = SERIAL.lock().expect("serial");
        let base = temp_base("mcp");
        let mcp_path = base.join("dsh-mcp.json");
        std::fs::write(
            &mcp_path,
            r#"{"mcpServers":{"user-own":{"command":"node","args":["a.js"],"env":{"TOKEN":"secret"}}}}"#,
        )
        .expect("seed mcp");
        std::env::set_var("DSH_MCP_PATH", &mcp_path);
        let def = BundleMcpServerDef {
            server_id: "mcp-e2e".into(),
            name: "E2E MCP".into(),
            transport: "stdio".into(),
            command: "npx".into(),
            args: vec!["-y".into(), "@modelcontextprotocol/server-github".into()],
            env_keys: vec!["GITHUB_TOKEN".into()],
            optional: false,
            description: String::new(),
        };
        merge_mcp_servers(&[def.clone()]).expect("合并失败");
        // 幂等：再次合并仍为两条
        let def2 = BundleMcpServerDef {
            server_id: "mcp-e2e-2".into(),
            name: "E2E MCP 2".into(),
            transport: "stdio".into(),
            command: "npx".into(),
            args: vec![],
            env_keys: vec![],
            optional: true,
            description: String::new(),
        };
        merge_mcp_servers(&[def, def2]).expect("二次合并失败");
        let merged = std::fs::read_to_string(&mcp_path).expect("读回失败");
        let v: serde_json::Value = serde_json::from_str(&merged).expect("合并结果非法 JSON");
        let servers = v
            .get("mcpServers")
            .and_then(|x| x.as_object())
            .expect("mcpServers 缺失");
        assert!(servers.contains_key("user-own"), "既有条目丢失: {}", merged);
        assert!(servers.contains_key("mcp-e2e"), "新条目未写入");
        assert!(servers.contains_key("mcp-e2e-2"), "幂等合并缺条目");
        let own_token = servers
            .get("user-own")
            .and_then(|s| s.get("env"))
            .and_then(|e| e.get("TOKEN"))
            .and_then(|t| t.as_str());
        assert_eq!(own_token, Some("secret"), "既有 env 值不应被改写");
        let new_env = servers
            .get("mcp-e2e")
            .and_then(|s| s.get("env"))
            .and_then(|e| e.get("GITHUB_TOKEN"))
            .and_then(|t| t.as_str());
        assert_eq!(new_env, Some(""), "env 只写键名+空值");
        std::env::remove_var("DSH_MCP_PATH");
        let _ = std::fs::remove_dir_all(&base);
    }
}
