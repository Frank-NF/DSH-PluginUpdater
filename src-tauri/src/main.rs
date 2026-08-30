#![windows_subsystem = "windows"]

mod error;
mod file_ops;
mod github_proxy;
mod manifest;
mod plugin_scan;
mod catalog;

use error::{AppConfig, AppError, AppResult, PluginInfo};
use file_ops::{open_in_file_manager, PluginFileManager};
use github_proxy::GitHubProxyClient;
use plugin_scan::scan_plugin_directory;
use std::fs;
use std::sync::Mutex;
use tauri::{Emitter, State};

struct AppState {
    config: Mutex<AppConfig>,
    plugins: Mutex<Vec<PluginInfo>>,
}


/// 返回官方目录全部插件（市场浏览用），不依赖本地安装
#[tauri::command]
async fn list_catalog_plugins(state: State<'_, AppState>) -> AppResult<Vec<error::MarketPlugin>> {
    let config = state.config.lock().unwrap().clone();
    let proxy = GitHubProxyClient::new(&config.proxy_base_url, None);

    let catalog = match catalog::get_catalog(proxy.http_client()).await {
        Ok(cat) => cat,
        Err(e) => {
            eprintln!("[market] 拉取插件目录失败: {}", e);
            return Ok(Vec::new());
        }
    };

    Ok(catalog
        .entries
        .into_iter()
        .map(|e| error::MarketPlugin {
            name: e.name,
            category: e.category,
            stars: e.stars,
            downloads: e.downloads,
            sha256: None,
            desc_zh: e.description.as_ref().and_then(|d| d.zh.clone()),
            desc_en: e.description.as_ref().and_then(|d| d.en.clone()),
            npm: e.npm,
            url: e.url,
        })
        .collect())
}

/// DSH 运行环境体检：检测安装目录/进程/运行时/插件目录/配置等
#[tauri::command]
fn check_environment(state: State<'_, AppState>) -> Vec<error::EnvCheckItem> {
    use std::path::PathBuf;
    let mut items: Vec<error::EnvCheckItem> = Vec::new();

    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let dsh_dir = home.join(".dsh");
    let profiles_dir = dsh_dir.join("profiles");
    let settings_path = dsh_dir.join("settings.yaml");

    // 1. DSH 安装目录
    if dsh_dir.exists() {
        items.push(error::EnvCheckItem {
            id: "dsh_dir".into(),
            name: "DSH 安装目录".into(),
            status: "ok".into(),
            message: format!("{} 存在", dsh_dir.display()),
            fix_hint: String::new(),
        });
    } else {
        items.push(error::EnvCheckItem {
            id: "dsh_dir".into(),
            name: "DSH 安装目录".into(),
            status: "error".into(),
            message: format!("未找到 {}，DSH 可能未安装", dsh_dir.display()),
            fix_hint: "请先安装 DSH（npm i -g @deepseek-ai/dsh），再重新运行本工具".into(),
        });
    }

    // 2. DSH 进程状态
    let running = is_dsh_running_impl();
    items.push(error::EnvCheckItem {
        id: "dsh_process".into(),
        name: "DSH 进程状态".into(),
        status: if running { "warn".into() } else { "ok".into() },
        message: if running {
            "检测到 DSH 正在运行".into()
        } else {
            "DSH 未运行".into()
        },
        fix_hint: if running {
            "更新插件前请先关闭 DSH 桌面端，否则运行时锁定插件目录会导致更新失败".into()
        } else {
            String::new()
        },
    });

    // 3. Node.js
    match std::process::Command::new("node").arg("--version").output() {
        Ok(out) if out.status.success() => {
            let ver = String::from_utf8_lossy(&out.stdout).trim().to_string();
            items.push(error::EnvCheckItem {
                id: "node".into(),
                name: "Node.js".into(),
                status: "ok".into(),
                message: format!("已安装 {}", ver),
                fix_hint: String::new(),
            });
        }
        _ => items.push(error::EnvCheckItem {
            id: "node".into(),
            name: "Node.js".into(),
            status: "error".into(),
            message: "未检测到 Node.js（node 不在 PATH）".into(),
            fix_hint: "请安装 Node.js（推荐 ≥ 18）后重启本工具".into(),
        }),
    }

    // 4. npm
    match std::process::Command::new("npm").arg("--version").output() {
        Ok(out) if out.status.success() => {
            let ver = String::from_utf8_lossy(&out.stdout).trim().to_string();
            items.push(error::EnvCheckItem {
                id: "npm".into(),
                name: "npm".into(),
                status: "ok".into(),
                message: format!("已安装 {}", ver),
                fix_hint: String::new(),
            });
        }
        _ => items.push(error::EnvCheckItem {
            id: "npm".into(),
            name: "npm".into(),
            status: "warn".into(),
            message: "未检测到 npm".into(),
            fix_hint: "部分插件安装依赖 npm，建议安装 Node.js 附带 npm".into(),
        }),
    }

    // 5. profiles 目录
    if profiles_dir.exists() {
        items.push(error::EnvCheckItem {
            id: "profiles".into(),
            name: "DSH profiles 目录".into(),
            status: "ok".into(),
            message: format!("{} 存在", profiles_dir.display()),
            fix_hint: String::new(),
        });
    } else {
        items.push(error::EnvCheckItem {
            id: "profiles".into(),
            name: "DSH profiles 目录".into(),
            status: "warn".into(),
            message: format!("未找到 {}，可能从未启动过 DSH", profiles_dir.display()),
            fix_hint: "启动一次 DSH 后会自动创建 profiles".into(),
        });
    }

    // 6. 配置文件
    if settings_path.exists() {
        items.push(error::EnvCheckItem {
            id: "settings".into(),
            name: "DSH 配置文件".into(),
            status: "ok".into(),
            message: format!("{} 存在", settings_path.display()),
            fix_hint: String::new(),
        });
    } else {
        items.push(error::EnvCheckItem {
            id: "settings".into(),
            name: "DSH 配置文件".into(),
            status: "warn".into(),
            message: format!("未找到 {}", settings_path.display()),
            fix_hint: "配置缺失可能导致模型接入异常，启动 DSH 生成后再检查".into(),
        });
    }

    // 7. 插件目录可写性
    let config = state.config.lock().unwrap().clone();
    let plugin_dir = config.plugin_directory.clone();
    if !plugin_dir.is_empty() {
        let p = std::path::Path::new(&plugin_dir);
        if p.exists() {
            let writable = std::fs::metadata(p).map(|md| !md.permissions().readonly()).unwrap_or(false);
            items.push(error::EnvCheckItem {
                id: "plugin_dir".into(),
                name: "插件目录".into(),
                status: if writable { "ok".into() } else { "warn".into() },
                message: format!("{}（{}）", plugin_dir, if writable { "可写" } else { "只读" }),
                fix_hint: if writable { String::new() } else { "目录只读会导致更新失败，请检查权限".into() },
            });
        } else {
            items.push(error::EnvCheckItem {
                id: "plugin_dir".into(),
                name: "插件目录".into(),
                status: "warn".into(),
                message: format!("{} 不存在", plugin_dir),
                fix_hint: "插件目录不存在，可在设置中重新指定".into(),
            });
        }
    } else {
        items.push(error::EnvCheckItem {
            id: "plugin_dir".into(),
            name: "插件目录".into(),
            status: "warn".into(),
            message: "尚未配置插件目录".into(),
            fix_hint: "在设置中指定插件目录或使用自动扫描".into(),
        });
    }

    items
}

/// 拼接 ssh 基础参数（密钥 + 端口 + BatchMode）
fn ssh_args(config: &AppConfig) -> Vec<String> {
    let mut v = vec![
        "ssh".to_string(),
        "-o".to_string(),
        "BatchMode=yes".to_string(),
        "-o".to_string(),
        "StrictHostKeyChecking=accept-new".to_string(),
        "-o".to_string(),
        "ConnectTimeout=8".to_string(),
    ];
    if !config.server_key.trim().is_empty() {
        v.push("-i".to_string());
        v.push(config.server_key.trim().to_string());
    }
    if config.server_port != 22 {
        v.push("-p".to_string());
        v.push(config.server_port.to_string());
    }
    v.push(format!("{}@{}", config.server_user, config.server_host));
    v
}

fn server_target(config: &AppConfig) -> String {
    format!("{}@{}", config.server_user, config.server_host)
}

/// 校验服务器配置是否填写
fn validate_server(config: &AppConfig) -> Result<(), String> {
    if config.server_host.trim().is_empty() || config.server_user.trim().is_empty() {
        return Err("请先在设置中填写服务器地址和用户名".into());
    }
    Ok(())
}

/// 测试 SSH 连接
#[tauri::command]
fn test_server_connection(state: State<'_, AppState>) -> Result<String, String> {
    let config = state.config.lock().unwrap().clone();
    validate_server(&config)?;

    let mut args = ssh_args(&config);
    args.push("echo DSH_SYNC_OK".to_string());

    let out = std::process::Command::new("ssh")
        .args(&args)
        .output()
        .map_err(|e| format!("无法启动 ssh: {}", e))?;

    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        Err(format!(
            "SSH 连接失败: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ))
    }
}

/// 同步到服务器：app=上传软件 exe；catalog=上传插件目录 JSON；plugins=远程执行插件更新命令
#[tauri::command]
async fn sync_to_server(
    kind: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let config = state.config.lock().unwrap().clone();
    validate_server(&config)?;

    match kind.as_str() {
        "app" => {
            let exe = std::env::current_exe()
                .map_err(|e| format!("无法定位当前 exe: {}", e))?;
            let target_dir = config.server_remote_dir.trim();
            if target_dir.is_empty() {
                return Err("请先在设置中填写远端发布目录".into());
            }
            // scp exe → user@host:remote_dir/
            let out = std::process::Command::new("scp")
                .args([
                    "-o", "BatchMode=yes",
                    "-o", "StrictHostKeyChecking=accept-new",
                    "-o", "ConnectTimeout=8",
                ])
                .arg("-P")
                .arg(config.server_port.to_string())
                .arg(&exe)
                .arg(format!("{}:{}/", server_target(&config), target_dir))
                .output()
                .map_err(|e| format!("无法启动 scp: {}", e))?;
            if out.status.success() {
                Ok(format!(
                    "软件已同步到服务器: {}/{}",
                    target_dir,
                    exe.file_name().unwrap_or_default().to_string_lossy()
                ))
            } else {
                Err(format!("同步失败: {}", String::from_utf8_lossy(&out.stderr).trim()))
            }
        }
        "catalog" => {
            // 拉取目录 → 序列化 MarketPlugin 列表 → 写临时 JSON → scp
            let proxy = GitHubProxyClient::new(&config.proxy_base_url, None);
            let catalog = catalog::get_catalog(proxy.http_client())
                .await
                .map_err(|e| format!("拉取插件目录失败: {}", e))?;
            let market: Vec<error::MarketPlugin> = catalog
                .entries
                .into_iter()
                .map(|e| error::MarketPlugin {
                    name: e.name,
                    category: e.category,
                    stars: e.stars,
                    downloads: e.downloads,
                    sha256: None,
                    desc_zh: e.description.as_ref().and_then(|d| d.zh.clone()),
                    desc_en: e.description.as_ref().and_then(|d| d.en.clone()),
                    npm: e.npm,
                    url: e.url,
                })
                .collect();
            let json = serde_json::to_string_pretty(&market).map_err(|e| e.to_string())?;

            let tmp = std::env::temp_dir().join("dsh-market-plugins.json");
            std::fs::write(&tmp, json).map_err(|e| e.to_string())?;

            let target_dir = config.server_remote_dir.trim();
            if target_dir.is_empty() {
                return Err("请先在设置中填写远端发布目录".into());
            }
            let out = std::process::Command::new("scp")
                .args([
                    "-o", "BatchMode=yes",
                    "-o", "StrictHostKeyChecking=accept-new",
                    "-o", "ConnectTimeout=8",
                ])
                .arg("-P")
                .arg(config.server_port.to_string())
                .arg(&tmp)
                .arg(format!("{}:{}/plugins.json", server_target(&config), target_dir))
                .output()
                .map_err(|e| format!("无法启动 scp: {}", e))?;
            let _ = std::fs::remove_file(&tmp);
            if out.status.success() {
                Ok(format!("插件目录（{} 个插件）已同步到服务器", market.len()))
            } else {
                Err(format!("同步失败: {}", String::from_utf8_lossy(&out.stderr).trim()))
            }
        }
        "plugins" => {
            // 远程执行插件更新命令（默认 cd 到 dsh 目录后执行）
            let dsh_dir = config.server_dsh_dir.trim();
            let cmd = if !config.server_update_cmd.trim().is_empty() {
                config.server_update_cmd.trim().to_string()
            } else if !dsh_dir.is_empty() {
                format!("cd {} && dsh plugin update --all -y", dsh_dir)
            } else {
                "dsh plugin update --all -y".to_string()
            };
            let mut args = ssh_args(&config);
            args.push(cmd);
            let out = std::process::Command::new("ssh")
                .args(&args)
                .output()
                .map_err(|e| format!("无法启动 ssh: {}", e))?;
            if out.status.success() {
                Ok(format!(
                    "服务器端插件更新命令已执行:\n{}",
                    String::from_utf8_lossy(&out.stdout).trim()
                ))
            } else {
                Err(format!(
                    "远程执行失败: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                ))
            }
        }
        _ => Err(format!("未知同步类型: {}", kind)),
    }
}
async fn build_catalog_map(
    client: &reqwest::Client,
) -> std::collections::HashMap<String, catalog::CatalogEntry> {
    match catalog::get_catalog(client).await {
        Ok(cat) => cat
            .entries
            .into_iter()
            .flat_map(|e| {
                let mut keys = vec![(e.name.to_lowercase(), e.clone())];
                if let Some(npm) = &e.npm {
                    keys.push((npm.to_lowercase(), e.clone()));
                }
                keys
            })
            .collect(),
        Err(_) => std::collections::HashMap::new(),
    }
}

/// 用官方目录元数据填充插件（双语描述 + 分类 + star + 下载量，无需任何翻译 API）
fn apply_catalog_metadata(
    catalog_map: &std::collections::HashMap<String, catalog::CatalogEntry>,
    plugins: &mut [PluginInfo],
) {
    for plugin in plugins.iter_mut() {
        if let Some(entry) = catalog_map.get(&plugin.manifest.id.to_lowercase()) {
            if let Some(d) = &entry.description {
                plugin.description_zh = d.zh.clone().filter(|z| !z.trim().is_empty());
                plugin.description_en = d.en.clone().filter(|s| !s.trim().is_empty());
            }
            plugin.category = entry.category.clone();
            plugin.stars = entry.stars;
            plugin.downloads = entry.downloads;
        }
    }
}

#[tauri::command]
async fn scan_plugins(directory: String, state: State<'_, AppState>) -> AppResult<Vec<PluginInfo>> {
    let mut plugins = scan_plugin_directory(&directory)?;

    // 扫描时即用官方目录填充元数据（描述/分类/star/下载），无需等检查更新
    {
        let config = state.config.lock().unwrap().clone();
        let proxy = GitHubProxyClient::new(&config.proxy_base_url, None);
        let catalog_map = build_catalog_map(proxy.http_client()).await;
    // 签名验证（如果服务器返回了签名）
    // 注：当前版本暂不强校验，仅记录签名状态
    // 生产环境可取消注释下方代码启用严格验证
    /*
    let sig_valid = true;
    if let Some(sig_header) = resp.headers().get("X-DSH-SIGNATURE") {
        let sig = sig_header.to_str().unwrap_or("");
        let pub_key = catalog::SIGNING_PUB_KEY;
        sig_valid = catalog::verify_catalog_signature(sig, &catalog_json, pub_key);
        if !sig_valid {
            eprintln!("[catalog] 签名验证失败，信任降级为缓存模式");
        }
    }
    */

        apply_catalog_metadata(&catalog_map, &mut plugins);
    }

    // 更新配置中的插件目录并持久化
    {
        let mut config = state.config.lock().unwrap();
        config.plugin_directory = directory.clone();
        let snapshot = config.clone();
        drop(config);
        let _ = save_config_to_disk(&snapshot);
    }

    // 保存到状态
    {
        let mut state_plugins = state.plugins.lock().unwrap();
        *state_plugins = plugins.clone();
    }

    Ok(plugins)
}

#[tauri::command]
async fn check_updates(state: State<'_, AppState>) -> AppResult<Vec<PluginInfo>> {
    let config = state.config.lock().unwrap().clone();
    let proxy = GitHubProxyClient::new(&config.proxy_base_url, None);

    let mut plugins = state.plugins.lock().unwrap().clone();

    // 先拉官方插件目录（npm 包源 → 官方 Pages fallback），建立 name/npm → entry 索引
    let catalog_map = build_catalog_map(proxy.http_client()).await;



    for plugin in plugins.iter_mut() {
        // 路线 1：目录声明了 npm 包 → npm registry 查最新版本（无 API 配额，国内镜像快）
        let npm_name = catalog_map
            .get(&plugin.manifest.id.to_lowercase())
            .and_then(|e| e.npm.clone());
        if let Some(npm_name) = npm_name.filter(|n| !n.is_empty()) {
            if let Ok((latest, tarball)) =
                catalog::npm_latest_meta(proxy.http_client(), &npm_name).await
            {
                let current = &plugin.manifest.current_version;
                let newer = match (semver::Version::parse(&latest), semver::Version::parse(current)) {
                    (Ok(l), Ok(c)) => l > c,
                    _ => latest != *current,
                };
                plugin.latest_version = Some(latest);
                plugin.download_url = tarball;
                plugin.update_available = newer;
                continue;
            }
        }

        // 路线 2：本地插件缺 repo 时，用目录条目补全 github_repo 再走现有 GitHub 检查
        if plugin.manifest.github_repo.is_empty() {
            if let Some(entry) = catalog_map.get(&plugin.manifest.id.to_lowercase()) {
                if let Some(url) = &entry.url {
                    if let Some(rest) = url.strip_prefix("https://github.com/") {
                        plugin.manifest.github_repo = rest.trim_matches('/').to_string();
                    }
                }
            }
        }

        // 官方目录元数据 → 双语描述 + 分类 + star + 下载量（无需任何翻译 API）
        apply_catalog_metadata(&catalog_map, std::slice::from_mut(plugin));
    }

    // 更新状态
    {
        let mut state_plugins = state.plugins.lock().unwrap();
        *state_plugins = plugins.clone();
    }

    Ok(plugins)
}

#[tauri::command]
async fn check_single_update(
    plugin_id: String,
    state: State<'_, AppState>,
) -> AppResult<PluginInfo> {
    let config = state.config.lock().unwrap().clone();
    let proxy = GitHubProxyClient::new(&config.proxy_base_url, None);

    // 先克隆插件数据，释放锁后再 await
    let mut plugin = {
        let plugins = state.plugins.lock().unwrap();
        plugins
            .iter()
            .find(|p| p.manifest.id == plugin_id)
            .ok_or_else(|| error::AppError::Other(format!("未找到插件: {}", plugin_id)))?
            .clone()
    };

    proxy.check_plugin_update(&mut plugin).await.map_err(|e| {
        error::AppError::Other(e)
    })?;

    // 更新状态
    {
        let mut plugins = state.plugins.lock().unwrap();
        if let Some(p) = plugins.iter_mut().find(|p| p.manifest.id == plugin_id) {
            *p = plugin.clone();
        }
    }

    Ok(plugin)
}

/// 列出可安装插件的目标目录（含 package.json 的 DSH profile 根）
#[tauri::command]
fn list_install_targets(state: State<'_, AppState>) -> Vec<String> {
    let mut targets: Vec<String> = Vec::new();
    if let Some(home) = dirs::home_dir() {
        let profiles = home.join(".dsh").join("profiles");
        if let Ok(rd) = fs::read_dir(&profiles) {
            for e in rd.flatten() {
                let p = e.path();
                let name = e.file_name().to_string_lossy().to_lowercase();
                if p.is_dir()
                    && !name.starts_with('.')
                    && !name.contains(".bak")
                    && p.join("package.json").is_file()
                {
                    let s = p.to_string_lossy().trim_end_matches(['\\', '/']).to_string();
                    if !targets.contains(&s) {
                        targets.push(s);
                    }
                }
            }
        }
    }
    if let Ok(cfg) = state.config.lock() {
        let d = cfg.plugin_directory.trim().to_string();
        if !d.is_empty() && std::path::Path::new(&d).join("package.json").is_file() && !targets.contains(&d) {
            targets.push(d);
        }
    }
    targets
}

/// 从 npm 安装插件到目标 DSH profile 目录（npm install --prefix）
#[tauri::command]
async fn install_plugin(
    npm_name: String,
    target_dir: String,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> AppResult<String> {
    let npm_name = npm_name.trim().to_string();
    if npm_name.is_empty() {
        return Err(error::AppError::Other("npm 包名为空".into()));
    }
    let root = std::path::Path::new(&target_dir);
    if !root.is_dir() {
        return Err(error::AppError::DirectoryNotFound(target_dir.clone()));
    }
    if !root.join("package.json").is_file() {
        return Err(error::AppError::Other(format!(
            "目标目录不是 DSH profile（缺少 package.json）: {}",
            target_dir
        )));
    }

    let emit = |phase: &str, message: String| {
        let _ = window.emit(
            "install_progress",
            serde_json::json!({ "npm_name": npm_name, "phase": phase, "message": message }),
        );
    };
    // 安装源：设置页可配（空 = npm 默认官方源），自定义须 http(s):// 开头
    let registry: String = {
        let cfg = state
            .config
            .lock()
            .map_err(|_| error::AppError::Other("配置锁不可用".into()))?;
        cfg.install_registry.trim().to_string()
    };
    if !registry.is_empty()
        && !registry.starts_with("http://")
        && !registry.starts_with("https://")
    {
        return Err(error::AppError::Other(format!(
            "安装源地址必须以 http:// 或 https:// 开头: {}",
            registry
        )));
    }

    emit("starting", format!("准备安装 {} ...", npm_name));
    if registry.is_empty() {
        emit(
            "installing",
            "正在执行 npm install（官方源），可能需要 1-3 分钟...".to_string(),
        );
    } else {
        emit(
            "installing",
            format!("正在执行 npm install（安装源: {}），可能需要 1-3 分钟...", registry),
        );
    }

    // Windows 经 cmd /c 调 npm（PATHEXT 解析 npm.cmd，绕过 PowerShell 执行策略）
    let mut cmd = if cfg!(windows) {
        let mut c = tokio::process::Command::new("cmd");
        c.arg("/c").arg("npm");
        c
    } else {
        tokio::process::Command::new("npm")
    };
    let mut npm_args: Vec<String> = vec![
        "install".into(),
        npm_name.clone(),
        "--prefix".into(),
        target_dir.clone(),
        "--no-audit".into(),
        "--no-fund".into(),
        // DSH 插件族（@deepseek-ai/*）peer 依赖互相冲突，npm7+ 默认严格解析会 ERESOLVE
        "--legacy-peer-deps".into(),
        "--loglevel".into(),
        "error".into(),
    ];
    if !registry.is_empty() {
        npm_args.push("--registry".into());
        npm_args.push(registry.clone());
    }
    cmd.args(&npm_args);
    cmd.current_dir(&target_dir);

    let output = tokio::time::timeout(std::time::Duration::from_secs(300), cmd.output())
        .await
        .map_err(|_| error::AppError::Other("npm install 超时（5 分钟）".into()))?
        .map_err(|e| {
            error::AppError::Other(format!(
                "启动 npm 失败: {}（请确认已安装 Node.js/npm 并在 PATH 中）",
                e
            ))
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !output.status.success() {
        let detail = if stderr.trim().is_empty() { stdout } else { stderr };
        let d = detail.trim().to_string();
        let detail: String = if d.chars().count() > 600 {
            d.chars().skip(d.chars().count() - 600).collect()
        } else {
            d
        };
        return Err(error::AppError::Other(format!("npm install 失败: {}", detail)));
    }

    emit("done", "安装完成".into());
    Ok(format!("已安装 {} 到 {}", npm_name, target_dir))
}
/// 枚举 DSH 相关进程（pid, 进程名）。排除本工具自身（dsh-plugin-updater）。
fn dsh_process_pids() -> Vec<(u32, String)> {
    use sysinfo::System;
    let sys = System::new_all();
    let mut pids: Vec<(u32, String)> = Vec::new();
    for p in sys.processes().values() {
        let name = p.name().to_string_lossy().to_lowercase();
        let is_dsh = (name.starts_with("dsh") && !name.starts_with("dsh-plugin-updater"))
            || name.contains("dsh desktop")
            || name.contains("dshdesktop");
        if is_dsh {
            pids.push((p.pid().as_u32(), p.name().to_string_lossy().to_string()));
        }
    }
    // 后备：sysinfo 在 Windows 上可能枚举失败（权限/版本差异），用 tasklist CSV 再确认
    if pids.is_empty() && cfg!(windows) {
        if let Ok(out) = std::process::Command::new("tasklist").args(["/FO", "CSV", "/NH"]).output() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                let lower = line.to_lowercase();
                if lower.contains("dsh desktop") || lower.contains("dshdesktop") || lower.contains("dsh.exe") {
                    let fields: Vec<&str> = line.split(',').collect();
                    if fields.len() >= 2 {
                        if let Ok(pid) = fields[1].trim_matches('"').parse::<u32>() {
                            let pname = fields[0].trim_matches('"').to_string();
                            if !pids.iter().any(|(p, _)| *p == pid) {
                                pids.push((pid, pname));
                            }
                        }
                    }
                }
            }
        }
    }
    pids
}

/// 检测 DSH 桌面端是否正在运行（运行时会锁定插件目录导致更新失败）
fn is_dsh_running_impl() -> bool {
    !dsh_process_pids().is_empty()
}

#[derive(serde::Serialize)]
struct DshProcessInfo {
    pid: u32,
    name: String,
}

/// 列出正在运行的 DSH 相关进程
#[tauri::command]
fn list_dsh_processes() -> Vec<DshProcessInfo> {
    dsh_process_pids()
        .into_iter()
        .map(|(pid, name)| DshProcessInfo { pid, name })
        .collect()
}

/// 强杀所有 DSH 相关进程（taskkill /F /T，含子进程树）。返回成功结束的数量。
/// 注意：本工具自身（dsh-plugin-updater）永远不会被误杀。
#[tauri::command]
fn kill_dsh_processes() -> AppResult<u32> {
    let pids = dsh_process_pids();
    let mut killed: u32 = 0;
    for (pid, _) in &pids {
        #[cfg(target_os = "windows")]
        {
            let ok = std::process::Command::new("taskkill")
                .args(["/F", "/T", "/PID", &pid.to_string()])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            if ok {
                killed += 1;
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let ok = std::process::Command::new("kill")
                .args(["-9", &pid.to_string()])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            if ok {
                killed += 1;
            }
        }
    }
    Ok(killed)
}

/// 用系统默认浏览器打开外部链接（仅允许 http/https）
#[tauri::command]
fn open_external(url: String) -> AppResult<()> {
    let url = url.trim().to_string();
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(error::AppError::Other("仅允许打开 http(s) 链接".into()));
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", "", &url])
            .spawn()
            .map_err(|e| error::AppError::Other(format!("打开链接失败: {}", e)))?;
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| error::AppError::Other(format!("打开链接失败: {}", e)))?;
    }
    Ok(())
}

/// 提权强杀 DSH 进程：经 PowerShell Start-Process -Verb RunAs 弹 UAC 授权框。
/// 标准用户也可在弹窗输入管理员密码完成（不要求预先以管理员运行本工具）。
/// 非阻塞：发出请求即返回尝试的进程数，由前端轮询 is_dsh_running 确认结果。
#[tauri::command]
fn kill_dsh_processes_elevated() -> AppResult<usize> {
    let pids = dsh_process_pids();
    if pids.is_empty() {
        return Ok(0);
    }
    let arg_list = pids
        .iter()
        .map(|(pid, _)| format!("/PID {}", pid))
        .collect::<Vec<_>>()
        .join(" ");
    let ps_cmd = format!(
        "Start-Process taskkill -Verb RunAs -ArgumentList '/F /T {}'",
        arg_list
    );
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_cmd])
            .spawn()
            .map_err(|e| error::AppError::Other(format!("无法发起提权请求: {}", e)))?;
    }
    Ok(pids.len())
}


#[tauri::command]
fn is_dsh_running() -> bool {
    is_dsh_running_impl()
}

#[tauri::command]
async fn update_plugin(
    plugin_id: String,
    window: tauri::Window,
    state: State<'_, AppState>,
    force: Option<bool>,
) -> AppResult<String> {
    // 防护：DSH 运行时会持有插件目录句柄，导致更新时文件重命名被锁失败
    if !force.unwrap_or(false) && is_dsh_running_impl() {
        return Err(error::AppError::Other(
            "检测到 DSH 桌面端正在运行。DSH 运行时会锁定插件目录，直接更新会失败。请先关闭 DSH 桌面端，或在前端确认弹窗中选择「仍要继续（不推荐）」。".to_string(),
        ));
    }

    let config = state.config.lock().unwrap().clone();
    let plugins = state.plugins.lock().unwrap().clone();

    let plugin = plugins
        .iter()
        .find(|p| p.manifest.id == plugin_id)
        .ok_or_else(|| error::AppError::Other(format!("未找到插件: {}", plugin_id)))?
        .clone();

    let download_url = plugin
        .download_url
        .clone()
        .ok_or_else(|| error::AppError::Other("未找到下载地址".to_string()))?;

    let latest_version = plugin
        .latest_version
        .clone()
        .ok_or_else(|| error::AppError::Other("未获取到最新版本".to_string()))?;

    let expected_sha256 = plugin.sha256.clone();
    let plugin_path = plugin.install_path.clone();
    let proxy = GitHubProxyClient::new(&config.proxy_base_url, None);
    let file_manager = PluginFileManager::new(&config.plugin_directory);

    // 发送进度事件
    let emit_progress = |phase: &str, percent: u8, message: &str| {
        let _ = window.emit(
            "update_progress",
            error::UpdateProgress {
                plugin_id: plugin_id.clone(),
                phase: phase.to_string(),
                percent,
                message: message.to_string(),
            },
        );
    };

    // 1. 备份旧插件
    let backup_path: Option<String> = if config.backup_before_update {
        emit_progress("backup", 5, "正在备份旧版本...");
        Some(file_manager
            .backup_plugin(&plugin_path, &plugin_id)
            .map_err(|e| error::AppError::Other(format!("备份失败: {}", e)))?
        )
    } else {
        None
    };

    // 2. 下载更新包
    emit_progress("download", 10, "正在下载更新包...");
    let temp_dir = std::env::temp_dir();
    let is_tgz = download_url.ends_with(".tgz") || download_url.ends_with(".tar.gz");
    let pkg_ext = if is_tgz { "tgz" } else { "zip" };
    // plugin_id 可能含 '/'（scoped 包如 @scope/name），拼文件名会变成不存在的目录
    let safe_id: String = plugin_id.chars().map(|c| if c == '/' || c == '\\' { '_' } else { c }).collect();
    let zip_path = temp_dir.join(format!("dsh_update_{}.{}", safe_id, pkg_ext));
    let zip_path_str = zip_path.to_string_lossy().to_string();

    let window_clone = window.clone();
    let plugin_id_clone = plugin_id.clone();
    proxy
        .download_file_with_progress(
            &download_url,
            &zip_path_str,
            move |downloaded, total| {
                let percent = if total > 0 {
                    (10.0 + (downloaded as f64 / total as f64) * 60.0) as u8
                } else {
                    40
                };
                let _ = window_clone.emit(
                    "update_progress",
                    error::UpdateProgress {
                        plugin_id: plugin_id_clone.clone(),
                        phase: "download".to_string(),
                        percent,
                        message: format!(
                            "下载中... {}/{}",
                            format_size(downloaded),
                            format_size(total)
                        ),
                    },
                );
            },
        )
        .await
        .map_err(|e| error::AppError::Other(format!("下载失败: {}", e)))?;

    // 3. SHA256 校验（如果服务器提供了预期值）
    if let Some(expected) = &expected_sha256 {
        emit_progress("verify", 70, "正在校验文件完整性...");
        let actual = file_ops::calculate_sha256(&zip_path_str)
            .map_err(|e| error::AppError::Other(format!("SHA256 计算失败: {}", e)))?;
        
        if actual != *expected {
            // 校验失败：清理临时文件，回滚备份
            file_ops::clean_temp_file(&zip_path_str);
            
            if let Some(backup) = &backup_path {
                emit_progress("rollback", 75, "文件校验失败，正在回滚...");
                if let Err(rollback_err) = file_manager.restore_backup(backup, &plugin_path) {
                    eprintln!("[update_plugin] 回滚失败: {}", rollback_err);
                }
            }
            
            return Err(error::AppError::Other(
                format!("文件完整性校验失败：期望 {:?}，实际 {:?}", expected, actual)
            ));
        }
        emit_progress("verify", 72, "校验通过");
    }


    // 3.5 兼容预检（官网源 fail-open：不可达时跳过，不阻塞安装）
    emit_progress("compat", 73, "正在检查兼容性...");
    let compat = catalog::compat_check(proxy.http_client(), &plugin_id, "0.1.1-rc.2").await;
    match compat {
        Ok((false, Some(note), _)) => {
            file_ops::clean_temp_file(&zip_path_str);
            if let Some(backup) = &backup_path {
                emit_progress("rollback", 74, "兼容性不满足，正在回滚...");
                let _ = file_manager.restore_backup(backup, &plugin_path);
            }
            return Err(error::AppError::CompatCheck(format!(
                "插件与该版本的 DSH 不兼容: {}", note
            )));
        }
        Ok((_, Some(note), conflicts)) if !conflicts.is_empty() => {
            // 有 warn 级冲突但没 blocking，发事件让前端提示，不 abort
            let _ = window.emit(
                "update_progress",
                serde_json::json!({
                    "plugin_id": plugin_id,
                    "phase": "compat_warn",
                    "percent": 73,
                    "message": format!("存在已知冲突: {}", note),
                }),
            );
        }
        Err(e) => {
            eprintln!("[update_plugin] compat check failed: {}", e);
        }
        _ => {}
    }
    emit_progress("compat", 74, "兼容检查通过");

    // 4. 解压更新包
    emit_progress("extract", 75, "正在解压更新包...");
    file_manager
        .extract_update_package(&zip_path_str, &plugin_path)
        .map_err(|e| error::AppError::Other(format!("解压失败: {}", e)))?;

    // 5. 更新版本号
    emit_progress("version", 90, "正在更新版本信息...");
    file_manager
        .update_plugin_version(&plugin_path, &latest_version)
        .map_err(|e| error::AppError::Other(format!("版本更新失败: {}", e)))?;

    // 6. 清理临时文件
    file_ops::clean_temp_file(&zip_path_str);

    // 6.5 同步 cordis profile 的依赖声明与锁文件（让 DSH 重启后识别新版本）
    {
        // node_modules/<pkg> 的上一级上一级 = profile 根（node_modules 目录）
        let mut profile_root: Option<std::path::PathBuf> = None;
        if let Some(p) = std::path::Path::new(&plugin_path).parent() {
            if let Some(pp) = p.parent() {
                if pp.join("package.json").exists() && pp.join("pnpm-lock.yaml").exists() {
                    profile_root = Some(pp.to_path_buf());
                }
            }
        }
        // npm 包名：scoped 用完整名，否则插件 id；从下载地址不可得，用 manifest.id（scoped 时含 @）
        let pkg_name = plugin.manifest.id.clone();
        if let Some(root) = profile_root {
            // a) 更新 profile package.json 的 dependencies 声明为 ^最新版
            let pkg_path = root.join("package.json");
            if let Ok(content) = fs::read_to_string(&pkg_path) {
                if let Ok(mut pkg) = serde_json::from_str::<serde_json::Value>(&content) {
                    let deps_changed = pkg["dependencies"][&pkg_name]
                        .as_str()
                        .map(|s| !s.starts_with("file:"))
                        .unwrap_or(false);
                    if deps_changed {
                        pkg["dependencies"][&pkg_name] =
                            serde_json::Value::String(format!("^{}", latest_version));
                        if let Ok(new_json) = serde_json::to_string_pretty(&pkg) {
                            let _ = fs::write(&pkg_path, new_json);
                        }
                    }
                }
            }
            // b) 更新 pnpm-lock.yaml 中该包的版本引用（specifier/键名/快照）
            let lock_path = root.join("pnpm-lock.yaml");
            if let Ok(lock_content) = fs::read_to_string(&lock_path) {
                let old_ver = &plugin.manifest.current_version;
                let new_ver = &latest_version;
                if old_ver != new_ver {
                    let mut updated = lock_content;
                    // 键名：pkg@old → pkg@new（含 scoped 引号键）
                    updated = updated.replace(&format!("{}@{}", pkg_name, old_ver), &format!("{}@{}", pkg_name, new_ver));
                    // specifier：pkg: ^old → pkg: ^new
                    updated = updated.replace(&format!("{}: ^{}", pkg_name, old_ver), &format!("{}: ^{}", pkg_name, new_ver));
                    // snapshot 里的 version: old(peer...) → new(peer...)
                    updated = updated.replace(&format!("version: {}(react", old_ver), &format!("version: {}(react", new_ver));
                    updated = updated.replace(&format!("version: {}(@deepseek", old_ver), &format!("version: {}(@deepseek", new_ver));
                    let _ = fs::write(&lock_path, updated);
                }
            }
        }
    }

    // 7. 更新内存中的插件信息
    {
        let mut state_plugins = state.plugins.lock().unwrap();
        if let Some(p) = state_plugins.iter_mut().find(|p| p.manifest.id == plugin_id) {
            p.manifest.current_version = latest_version.clone();
            p.update_available = false;
            p.latest_version = Some(latest_version.clone());
        }
    }

    emit_progress("complete", 100, "更新完成！");
    Ok(latest_version)
}

/// 检查桌面端自身是否有新版本
#[tauri::command]
async fn check_self_update() -> AppResult<SelfUpdateInfo> {
    use reqwest::Client;
    let client = Client::builder().timeout(std::time::Duration::from_secs(10)).build().unwrap_or_default();
    let resp = match client.get("https://dsh.huilinsh.cn/api/updater/latest").send().await {
        Ok(r) => r, Err(e) => { eprintln!("[self_update] 请求失败: {}", e); return Ok(SelfUpdateInfo { available: false, current_version: env!("CARGO_PKG_VERSION").to_string(), latest_version: None, changelog: vec![], release_url: None, is_mandatory: false }); }
    };
    if !resp.status().is_success() { return Ok(SelfUpdateInfo { available: false, current_version: env!("CARGO_PKG_VERSION").to_string(), latest_version: None, changelog: vec![], release_url: None, is_mandatory: false }); }
    #[derive(serde::Deserialize)]
    struct UR { version: String, #[serde(default)] changelog: Vec<String>, #[serde(default)] release_url: Option<String>, #[serde(default)] is_mandatory: bool }
    let data: UR = match resp.json().await { Ok(d) => d, Err(e) => { eprintln!("[self_update] 解析失败: {}", e); return Ok(SelfUpdateInfo { available: false, current_version: env!("CARGO_PKG_VERSION").to_string(), latest_version: None, changelog: vec![], release_url: None, is_mandatory: false }); } };
    let current = env!("CARGO_PKG_VERSION").to_string();
    let latest = data.version.clone();
    let available = match (semver::Version::parse(&current), semver::Version::parse(&latest)) { (Ok(a), Ok(b)) => b > a, _ => false };
    Ok(SelfUpdateInfo { available, current_version: current, latest_version: Some(latest), changelog: data.changelog, release_url: data.release_url, is_mandatory: data.is_mandatory })
}

/// 执行自我更新：下载新版 exe 并替换
#[tauri::command]
async fn self_update(window: tauri::Window) -> AppResult<String> {
    let info = check_self_update().await?;
    if !info.available { return Err(AppError::SelfUpdate("当前已是最新版本".to_string())); }
    let latest = match info.latest_version {
            Some(v) => v,
            None => return Err(AppError::SelfUpdate("版本信息缺失".to_string())),
        };
    let url = info.release_url.unwrap_or("https://dsh.huilinsh.cn/dsh-plugin-updater.exe".to_string());
    let temp = std::env::temp_dir().join(format!("dsh-updater-{}.exe", latest));
    let temp_str = temp.to_string_lossy().to_string();
    emit_self_progress(&window, "download", 10, "正在下载新版本...");
    let bytes = reqwest::Client::new().get(&url).send().await.map_err(|e| AppError::SelfUpdate(e.to_string()))?.bytes().await.map_err(|e| AppError::SelfUpdate(e.to_string()))?;
    std::fs::write(&temp, &bytes).map_err(|e| AppError::SelfUpdate(e.to_string()))?;
    emit_self_progress(&window, "launch", 80, "正在启动更新进程...");
    let current = std::env::current_exe().map_err(|e| AppError::SelfUpdate(e.to_string()))?;
    std::process::Command::new("cmd").args(["/c", "timeout", "/t", "2", "/nobreak >nul", "&", "move", "/Y", &temp_str, &current.to_string_lossy(), "&", "start", "", &current.to_string_lossy()]).spawn().map_err(|e| AppError::SelfUpdate(e.to_string()))?;
    std::process::exit(0);
}

fn emit_self_progress(w: &tauri::Window, phase: &str, pct: u8, msg: &str) {
    let _ = w.emit("self_update_progress", serde_json::json!({ "phase": phase, "percent": pct, "message": msg }));
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SelfUpdateInfo { pub available: bool, pub current_version: String, pub latest_version: Option<String>, pub changelog: Vec<String>, pub release_url: Option<String>, pub is_mandatory: bool }
#[tauri::command]
fn uninstall_plugin(plugin_id: String, state: State<'_, AppState>) -> AppResult<()> {
    let config = state.config.lock().unwrap().clone();
    let plugins = state.plugins.lock().unwrap().clone();

    let plugin = plugins
        .iter()
        .find(|p| p.manifest.id == plugin_id)
        .ok_or_else(|| error::AppError::Other(format!("未找到插件: {}", plugin_id)))?;

    // Agent 本体不允许卸载
    if plugin.manifest.r#type == "agent-core" {
        return Err(error::AppError::Other("Agent本体不允许卸载".to_string()));
    }

    let file_manager = PluginFileManager::new(&config.plugin_directory);
    file_manager
        .uninstall_plugin(&plugin.install_path, &plugin_id)
        .map_err(|e| error::AppError::Other(format!("卸载失败: {}", e)))?;

    // 从状态中移除
    {
        let mut state_plugins = state.plugins.lock().unwrap();
        state_plugins.retain(|p| p.manifest.id != plugin_id);
    }

    Ok(())
}

#[tauri::command]
fn set_plugin_enabled(
    plugin_id: String,
    enabled: bool,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let config = state.config.lock().unwrap().clone();
    let mut plugins = state.plugins.lock().unwrap();

    let plugin = plugins
        .iter_mut()
        .find(|p| p.manifest.id == plugin_id)
        .ok_or_else(|| error::AppError::Other(format!("未找到插件: {}", plugin_id)))?;

    let file_manager = PluginFileManager::new(&config.plugin_directory);
    file_manager
        .set_plugin_enabled(&plugin.install_path, &mut plugin.manifest, enabled)
        .map_err(|e| error::AppError::Other(format!("操作失败: {}", e)))?;

    Ok(())
}

#[tauri::command]
fn open_plugin_folder(plugin_id: String, state: State<'_, AppState>) -> AppResult<()> {
    let plugins = state.plugins.lock().unwrap();
    let plugin = plugins
        .iter()
        .find(|p| p.manifest.id == plugin_id)
        .ok_or_else(|| error::AppError::Other(format!("未找到插件: {}", plugin_id)))?;

    open_in_file_manager(&plugin.install_path)?;
    Ok(())
}

#[tauri::command]
fn get_config(state: State<'_, AppState>) -> AppConfig {
    state.config.lock().unwrap().clone()
}

/// 配置文件路径：~/.dsh/plugin-updater-config.json
fn config_path() -> std::path::PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".dsh")
        .join("plugin-updater-config.json")
}

/// 启动时从磁盘加载配置；文件不存在或损坏时用默认值
fn load_config_from_disk() -> AppConfig {
    let path = config_path();
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(cfg) = serde_json::from_str::<AppConfig>(&content) {
            return cfg;
        }
    }
    AppConfig::default()
}

/// 配置写盘（原子性：先写临时文件再改名，避免写一半损坏）
fn save_config_to_disk(cfg: &AppConfig) -> AppResult<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(cfg)?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json)?;
    fs::rename(&tmp, &path)?;
    Ok(())
}

#[tauri::command]
fn update_config(new_config: AppConfig, state: State<'_, AppState>) -> AppResult<()> {
    {
        let mut config = state.config.lock().unwrap();
        *config = new_config.clone();
    }
    // 持久化：修复设置重启丢失（安装源/代理/插件目录/服务器同步等全部字段）
    save_config_to_disk(&new_config)?;
    Ok(())
}

#[tauri::command]
fn list_backups(state: State<'_, AppState>) -> AppResult<Vec<file_ops::BackupInfo>> {
    let config = state.config.lock().unwrap().clone();
    let file_manager = PluginFileManager::new(&config.plugin_directory);
    file_manager.list_backups()
}

#[tauri::command]
fn restore_backup(
    backup_path: String,
    plugin_id: String,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let config = state.config.lock().unwrap().clone();
    let plugins = state.plugins.lock().unwrap();
    let plugin = plugins
        .iter()
        .find(|p| p.manifest.id == plugin_id)
        .ok_or_else(|| error::AppError::Other(format!("未找到插件: {}", plugin_id)))?;

    let file_manager = PluginFileManager::new(&config.plugin_directory);
    file_manager.restore_backup(&backup_path, &plugin.install_path)?;
    Ok(())
}

#[tauri::command]
fn validate_directory(path: String) -> Result<(), String> {
    plugin_scan::validate_plugin_directory(&path)
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}



#[tauri::command]
async fn auto_scan_plugins(state: State<'_, AppState>) -> AppResult<Vec<PluginInfo>> {
    let config = state.config.lock().unwrap().clone();
    
    // 如果已有配置的插件目录，直接使用
    if !config.plugin_directory.is_empty() {
        let scanned = scan_plugin_directory(&config.plugin_directory)?;
        if !scanned.is_empty() {
            return scan_plugins(config.plugin_directory, state).await;
        }
    }
    
    // 构建候选插件目录列表
    let home = dirs::home_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
    let data_local = dirs::data_local_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
    
    let mut candidates: Vec<String> = Vec::new();
    
    // 1. DSH profile 目录（实际插件安装位置）
    for profile in ["web", "desktop", "cli", "mobile"] {
        candidates.push(format!("{}/.dsh/profiles/{}", home, profile));
        candidates.push(format!("{}/.dsh/profiles/{}/plugin-sources", home, profile));
    }
    // 2. 直接指向插件集合的目录
    candidates.push(format!("{}/.dsh/profiles", home));
    candidates.push(format!("{}/.dsh/plugins", home));
    candidates.push(format!("{}/.dsh/extensions", home));
    // 3. 常见目录
    candidates.push(format!("{}/DSH/plugins", data_local));
    candidates.push(format!("{}/DSH-PluginUpdater/plugins", data_local));
    candidates.push("C:/DSH/plugins".to_string());
    candidates.push("C:/Program Files/DSH/plugins".to_string());
    
    let mut last_error: Option<String> = None;
    
    for path in &candidates {
        let p = std::path::Path::new(&path);
        if !p.exists() || !p.is_dir() {
            continue;
        }
        
        match scan_plugin_directory(&path) {
            Ok(plugins) if !plugins.is_empty() => {
                // 找到插件，保存配置并返回
                return scan_plugins(path.clone(), state).await;
            }
            Ok(_) => {
                last_error = Some(format!("{} 无插件", path));
            }
            Err(e) => {
                last_error = Some(e.to_string());
            }
        }
    }
    
    let detail = last_error.unwrap_or_default();
    Err(AppError::Other(format!("未找到插件目录（已尝试 {} 个位置：{}），请在设置中手动指定", candidates.len(), detail)))
}
/// 弹出系统目录选择框，返回所选路径（取消返回 None）
#[tauri::command]
async fn pick_directory(window: tauri::WebviewWindow) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let (tx, rx) = tokio::sync::oneshot::channel::<Option<String>>();
    window
        .dialog()
        .file()
        .set_title("选择插件目录")
        .pick_folder(move |path| {
            let _ = tx.send(path.map(|p| p.to_string()));
        });
    rx.await.map_err(|e| e.to_string())
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            // 从 ~/.dsh/plugin-updater-config.json 加载，修复设置重启丢失
            config: Mutex::new(load_config_from_disk()),
            plugins: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            pick_directory,
            scan_plugins,
            auto_scan_plugins,
            check_updates,
            check_single_update,
            update_plugin,
            uninstall_plugin,
            set_plugin_enabled,
            open_plugin_folder,
            get_config,
            update_config,
            list_backups,
            restore_backup,
            is_dsh_running,
            validate_directory,
            list_catalog_plugins,
            check_environment,
            install_plugin,
            list_dsh_processes,
            kill_dsh_processes,
            open_external,
            kill_dsh_processes_elevated,
            list_install_targets,
            check_self_update,
            self_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running DSH Plugin Updater");
}
