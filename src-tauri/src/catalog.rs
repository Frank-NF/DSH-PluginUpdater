use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::time::{Duration, Instant};

const TIMEOUT: Duration = Duration::from_secs(20);

/// npm 镜像列表（顺序尝试，腾讯镜像国内最快）
const NPM_MIRRORS: &[&str] = &[
    "https://mirrors.cloud.tencent.com/npm",
    "https://registry.npmjs.org",
];

const CATALOG_PACKAGE: &str = "dsh-plugin-catalog";
const CATALOG_OFFICIAL_URL: &str = "https://awesome-dsh-plugin.com/plugins.json";
/// 官网权威源（商用化统一数据入口，Phase 1）
const CATALOG_WEBSITE_URL: &str = "https://dsh.huilinsh.cn/api/plugins?fields=full&page_size=200";
const CATALOG_TTL: Duration = Duration::from_secs(600);

/// 编译时嵌入的签名公钥（32 bytes Ed25519）
pub const SIGNING_PUB_KEY: &[u8] = include_bytes!("../keys/ed25519-public.bin");


/// 本地磁盘缓存路径（官网不可达时兜底，不白屏）
const CACHE_DIR_NAME: &str = "dsh-plugin-updater";

/// 官方插件目录的一条记录
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatalogEntry {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub npm: Option<String>,
    #[serde(default)]
    pub description: Option<CatalogDescription>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub stars: Option<u64>,
    #[serde(default)]
    pub downloads: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatalogDescription {
    #[serde(default)]
    pub en: Option<String>,
    #[serde(default)]
    pub zh: Option<String>,
}

#[derive(Deserialize)]
struct CatalogFile {
    #[serde(default)]
    plugins: Vec<CatalogEntry>,
}

/// 已加载的目录（含获取时间，用于 TTL 缓存；sig_valid 为 None 表示未验证/无签名头）
pub struct Catalog {
    pub entries: Vec<CatalogEntry>,
    pub fetched_at: Instant,
    pub source: String,
    /// 签名验证结果：Some(true)=验证通过，Some(false)=验证失败（不可信），None=服务器未返回签名头
    pub sig_valid: Option<bool>,
}

impl Catalog {
    /// 按插件名（npm 名或 repo 名）查找目录条目
    pub fn find(&self, plugin_name: &str) -> Option<&CatalogEntry> {
        let lower = plugin_name.to_lowercase();
        self.entries.iter().find(|e| {
            e.name.to_lowercase() == lower
                || e.npm
                    .as_ref()
                    .map(|n| n.to_lowercase() == lower)
                    .unwrap_or(false)
        })
    }
}

/// 从 npm 包 tarball 解出 plugins.json
fn plugins_json_from_tarball(gz_bytes: &[u8]) -> AppResult<Vec<u8>> {
    let gz = flate2::read::GzDecoder::new(gz_bytes);
    let mut archive = tar::Archive::new(gz);
    for entry in archive.entries().map_err(|e| AppError::Other(format!("tar 解析失败: {}", e)))? {
        let mut entry = entry.map_err(|e| AppError::Other(format!("tar 条目读取失败: {}", e)))?;
        let path = entry
            .path()
            .map_err(|e| AppError::Other(format!("tar 路径读取失败: {}", e)))?
            .to_string_lossy()
            .to_string();
        if path == "package/plugins.json" {
            let mut buf = Vec::new();
            entry
                .read_to_end(&mut buf)
                .map_err(|e| AppError::Other(format!("plugins.json 读取失败: {}", e)))?;
            return Ok(buf);
        }
    }
    Err(AppError::Other("npm 包内未找到 package/plugins.json".to_string()))
}

/// 官网 API 响应结构（fields=basic 精简模式）
#[derive(Debug, Deserialize)]
struct WebsiteCatalogResponse {
    #[serde(default)]
    total: Option<usize>,
    #[serde(default)]
    plugins: Vec<WebsitePluginItem>,
}

#[derive(Debug, Deserialize)]
struct WebsitePluginItem {
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    stars: Option<u64>,
    #[serde(default)]
    github_url: Option<String>,
    /// 官网返回的展示描述（zh 优先的合并串）
    #[serde(default)]
    description: Option<String>,
    /// GitHub 原始英文描述
    #[serde(default)]
    github_description: Option<String>,
    /// npm 月下载量（官网 full 模式透传）
    #[serde(default)]
    downloads: Option<u64>,
}

/// 本地磁盘缓存路径：%APPDATA%/dsh-plugin-updater/catalog.json
fn cache_file_path() -> Option<std::path::PathBuf> {
    let dir = dirs::config_dir()?;
    let cache_dir = dir.join(CACHE_DIR_NAME);
    std::fs::create_dir_all(&cache_dir).ok()?;
    Some(cache_dir.join("catalog.json"))
}

/// 写入磁盘缓存（官网源成功时刷新）
fn write_cache(entries: &[CatalogEntry]) {
    if let Some(path) = cache_file_path() {
        if let Ok(json) = serde_json::to_string(entries) {
            let _ = std::fs::write(path, json);
        }
    }
}

/// 读取磁盘缓存（官网不可达时兜底）
fn read_cache() -> Option<Vec<CatalogEntry>> {
    let path = cache_file_path()?;
    let raw = std::fs::read_to_string(path).ok()?;
    let entries: Vec<CatalogEntry> = serde_json::from_str(&raw).ok()?;
    if entries.is_empty() { None } else { Some(entries) }
}

/// 从官网权威源拉取目录（Phase 1：统一数据入口；服务端 page_size 上限 200，循环分页拉全量）
pub async fn fetch_catalog_from_website(client: &reqwest::Client) -> AppResult<Catalog> {
    let mut items: Vec<WebsitePluginItem> = Vec::new();
    let mut expected_total: Option<usize> = None;
    let mut all_sig_valid: Option<bool> = None;
    for page in 1..=30usize {
        let sep = if CATALOG_WEBSITE_URL.contains('?') { '&' } else { '?' };
        let url = format!("{}{}page={}", CATALOG_WEBSITE_URL, sep, page);
        let (parsed, sig_valid) = verify_page_signature(client, &url).await?;
        if expected_total.is_none() {
            expected_total = parsed.total;
        }
        // 累积签名结果：第一页有签名则后续页也应有，全部通过才为 true
        if let Some(sv) = sig_valid {
            match all_sig_valid {
                None => all_sig_valid = Some(sv),
                Some(prev) => { if !sv { all_sig_valid = Some(false); } }
            }
        }
        let got = parsed.plugins.len();
        items.extend(parsed.plugins);
        if got == 0 {
            break;
        }
        if let Some(t) = expected_total {
            if items.len() >= t {
                break;
            }
        }
    }
    if items.is_empty() {
        return Err(AppError::Other("官网目录为空".to_string()));
    }

    let mut entries: Vec<CatalogEntry> = items
        .into_iter()
        .map(|p| {
            // 官网 description = zh || en 合并串；github_description = 英文原文
            let (zh, en) = match (p.description.clone(), p.github_description.clone()) {
                (Some(z), Some(e)) => (Some(z), Some(e)),
                (Some(z), None) => (Some(z.clone()), Some(z)),
                (None, Some(e)) => (None, Some(e)),
                (None, None) => (None, None),
            };
            CatalogEntry {
                name: p.id.clone(),
                url: p.github_url,
                npm: Some(p.id),
                description: if zh.is_some() || en.is_some() {
                    Some(CatalogDescription { en, zh })
                } else {
                    None
                },
                category: p.category,
                stars: p.stars,
                downloads: p.downloads,
            }
        })
        .collect();

    // 尽力而为补 npm 月下载量（并发 8 路，失败静默不阻塞目录）
    enrich_npm_downloads(client, &mut entries).await;

    // 成功时刷新磁盘缓存
    write_cache(&entries);

    Ok(Catalog {
        entries,
        fetched_at: Instant::now(),
        source: "dsh.huilinsh.cn".to_string(),
    })
}

/// 尽力而为补全 npm 月下载量（并发 8 路，单包失败静默跳过，总体限时）
async fn enrich_npm_downloads(client: &reqwest::Client, entries: &mut [CatalogEntry]) {
    // 官网目录已自带 npm 月下载量（downloads 字段）→ 仅对缺失的条目兜底补拉；
    // 此前无条件对全部条目发起 api.npmjs.org 请求（目录 2100+ 条全部命中），
    // 每次目录刷新触发 20s 预算的请求风暴且永远拉不完——列表加载慢的主因。
    let targets: Vec<(usize, String)> = entries
        .iter()
        .enumerate()
        .filter_map(|(i, e)| match (&e.npm, e.downloads) {
            (Some(n), None) | (Some(n), Some(0)) => Some((i, n.clone())),
            _ => None,
        })
        .collect();
    if targets.is_empty() {
        return;
    }

    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(8));
    let mut handles = Vec::with_capacity(targets.len());
    for (idx, npm) in targets {
        let sem = semaphore.clone();
        let client = client.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.ok()?;
            let url = format!(
                "https://api.npmjs.org/downloads/point/last-month/{}",
                npm
            );
            let resp = client
                .get(&url)
                .header("User-Agent", "dsh-plugin-updater")
                .header("Accept", "application/json")
                .timeout(std::time::Duration::from_secs(6))
                .send()
                .await
                .ok()?;
            if !resp.status().is_success() {
                return None;
            }
            #[derive(serde::Deserialize)]
            struct NpmDownloads {
                downloads: u64,
            }
            let parsed: NpmDownloads = resp.json().await.ok()?;
            Some((idx, parsed.downloads))
        }));
    }

    // 总预算 20s：到点放弃剩余包（下次 TTL 刷新或重启会补全）
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(20);
    for handle in handles {
        if tokio::time::Instant::now() >= deadline {
            handle.abort();
            continue;
        }
        let remaining = deadline - tokio::time::Instant::now();
        match tokio::time::timeout(remaining, handle).await {
            Ok(Ok(Some((idx, downloads)))) => {
                entries[idx].downloads = Some(downloads);
            }
            _ => {}
        }
    }
}

/// 拉取官方插件目录。
/// 源顺序：官网权威源 → npm 镜像包（腾讯镜像 → npmjs）→ 官方 GitHub Pages 直连 → 本地磁盘缓存。
pub async fn fetch_catalog(client: &reqwest::Client) -> AppResult<Catalog> {
    // 0. 优先官网权威源（统一数据入口）
    match fetch_catalog_from_website(client).await {
        Ok(cat) => return Ok(cat),
        Err(e) => eprintln!("[catalog] 官网源失败，降级 npm/Pages: {}", e),
    }

    let mut last_err: Option<String> = None;

    // 路线 1：npm 包（镜像 rewritten dist.tarball，国内走镜像）
    for mirror in NPM_MIRRORS {
        let meta_url = format!("{}/{}/latest", mirror, CATALOG_PACKAGE);
        match client.get(&meta_url).timeout(TIMEOUT).send().await {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(meta) => {
                        let tarball = meta["dist"]["tarball"].as_str().unwrap_or("").to_string();
                        let version = meta["version"].as_str().unwrap_or("?").to_string();
                        if tarball.is_empty() {
                            last_err = Some(format!("{} 元数据无 tarball", mirror));
                            continue;
                        }
                        match client.get(&tarball).timeout(TIMEOUT).send().await {
                            Ok(tg) if tg.status().is_success() => {
                                match tg.bytes().await {
                                    Ok(bytes) => match plugins_json_from_tarball(&bytes) {
                                        Ok(json) => {
                                            let parsed: CatalogFile = serde_json::from_slice(&json)
                                                .map_err(|e| AppError::Other(format!("目录解析失败: {}", e)))?;
                                            if parsed.plugins.is_empty() {
                                                last_err = Some(format!("{} 目录为空", mirror));
                                                continue;
                                            }
                                            return Ok(Catalog {
                                                entries: parsed.plugins,
                                                fetched_at: Instant::now(),
                                                source: format!("{}@{}", CATALOG_PACKAGE, version),
                                            });
                                        }
                                        Err(e) => last_err = Some(e.to_string()),
                                    },
                                    Err(e) => last_err = Some(format!("tarball 下载失败: {}", e)),
                                }
                            }
                            Ok(tg) => last_err = Some(format!("tarball HTTP {}", tg.status())),
                            Err(e) => last_err = Some(format!("tarball 请求失败: {}", e)),
                        }
                    }
                    Err(e) => last_err = Some(format!("{} 元数据解析失败: {}", mirror, e)),
                }
            }
            Ok(resp) => last_err = Some(format!("{} HTTP {}", mirror, resp.status())),
            Err(e) => last_err = Some(format!("{} 请求失败: {}", mirror, e)),
        }
    }

    // 路线 2：官方 Pages 直连
    match client.get(CATALOG_OFFICIAL_URL).timeout(TIMEOUT).send().await {
        Ok(resp) if resp.status().is_success() => match resp.bytes().await {
            Ok(bytes) => {
                let parsed: CatalogFile = serde_json::from_slice(&bytes)
                    .map_err(|e| AppError::Other(format!("官方目录解析失败: {}", e)))?;
                if !parsed.plugins.is_empty() {
                    return Ok(Catalog {
                        entries: parsed.plugins,
                        fetched_at: Instant::now(),
                        source: "awesome-dsh-plugin.com".to_string(),
                    });
                }
                last_err = Some("官方目录为空".to_string());
            }
            Err(e) => last_err = Some(format!("官方目录下载失败: {}", e)),
        },
        Ok(resp) => last_err = Some(format!("官方目录 HTTP {}", resp.status())),
        Err(e) => last_err = Some(format!("官方目录请求失败: {}", e)),
    }

    // 最后兜底：本地磁盘缓存（断网/内网场景不白屏）
    if let Some(cached) = read_cache() {
        eprintln!("[catalog] 所有网络源不可达，使用本地缓存 ({} 条)", cached.len());
        return Ok(Catalog {
            entries: cached,
            fetched_at: Instant::now(),
            source: "disk-cache".to_string(),
        });
    }

    Err(AppError::Other(format!(
        "所有目录源均不可达，最后错误: {}",
        last_err.unwrap_or_default()
    )))
}

/// npm registry 查询包的最新版本（无 API 配额限制）

/// 查询 npm 包最新版本及其 tarball 下载地址（腾讯镜像优先）
/// 批量检查结果（官网 batch-check 端点，一次请求替代逐插件串行查询）
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCheckResult {
    pub id: String,
    #[serde(default)]
    pub latest: Option<String>,
    #[serde(default)]
    pub tarball: Option<String>,
    #[serde(default)]
    pub sha: Option<String>,
    #[serde(default)]
    pub update_available: bool,
}

/// 官网批量检查更新：POST /api/updater/batch-check
/// 服务端机房直连 npm registry，客户端只需一次国内请求（约 0.3s）
pub async fn batch_check_website(
    client: &reqwest::Client,
    items: &[(String, String, String)],
) -> AppResult<Vec<BatchCheckResult>> {
    #[derive(serde::Serialize)]
    struct Item<'a> {
        id: &'a str,
        npm: &'a str,
        version: &'a str,
    }
    let payload: Vec<Item> = items
        .iter()
        .map(|(id, npm, ver)| Item { id, npm, version: ver })
        .collect();
    let resp = client
        .post("https://dsh.huilinsh.cn/api/updater/batch-check")
        .timeout(Duration::from_secs(12))
        .json(&serde_json::json!({ "items": payload }))
        .send()
        .await
        .map_err(|e| AppError::Other(format!("batch-check 请求失败: {}", e)))?;
    if !resp.status().is_success() {
        return Err(AppError::Other(format!("batch-check HTTP {}", resp.status())));
    }
    #[derive(Deserialize)]
    struct Resp {
        results: Vec<BatchCheckResult>,
    }
    let parsed: Resp = resp.json().await?;
    Ok(parsed.results)
}

pub async fn npm_latest_meta(
    client: &reqwest::Client,
    npm_name: &str,
) -> AppResult<(String, Option<String>, Option<String>)> {
    let encoded = urlencoding::encode(npm_name);
    let mut last_err: Option<String> = None;
    for mirror in NPM_MIRRORS {
        let url = format!("{}/{}/latest", mirror, encoded);
        match client.get(&url).timeout(TIMEOUT).send().await {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(meta) => {
                        if let Some(v) = meta["version"].as_str() {
                            let tarball = meta["dist"]["tarball"]
                                .as_str()
                                .map(|s| s.to_string());
                            // dist.shasum = 发布时登记的 SHA1-hex，用于下载后完整性核对
                            let shasum = meta["dist"]["shasum"]
                                .as_str()
                                .map(|s| s.to_string());
                            return Ok((v.to_string(), tarball, shasum));
                        }
                        last_err = Some(format!("{} 响应无 version", mirror));
                    }
                    Err(e) => last_err = Some(format!("{} 解析失败: {}", mirror, e)),
                }
            }
            Ok(resp) => last_err = Some(format!("{} HTTP {}", mirror, resp.status())),
            Err(e) => last_err = Some(format!("{} 请求失败: {}", mirror, e)),
        }
    }
    Err(AppError::Other(last_err.unwrap_or_else(|| "npm 查询失败".into())))
}
pub async fn npm_latest_version(client: &reqwest::Client, npm_name: &str) -> AppResult<String> {
    let encoded = urlencoding::encode(npm_name);
    let mut last_err: Option<String> = None;
    for mirror in NPM_MIRRORS {
        let url = format!("{}/{}/latest", mirror, encoded);
        match client.get(&url).timeout(TIMEOUT).send().await {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(meta) => {
                        if let Some(v) = meta["version"].as_str() {
                            return Ok(v.to_string());
                        }
                        last_err = Some(format!("{} 响应无 version", mirror));
                    }
                    Err(e) => last_err = Some(format!("{} 解析失败: {}", mirror, e)),
                }
            }
            Ok(resp) => last_err = Some(format!("{} HTTP {}", mirror, resp.status())),
            Err(e) => last_err = Some(format!("{} 请求失败: {}", mirror, e)),
        }
    }
    Err(AppError::Other(format!(
        "npm 最新版本查询失败({}): {}",
        npm_name,
        last_err.unwrap_or_default()
    )))
}

/// 全局目录缓存（进程级，TTL 10 分钟）
static CATALOG_CACHE: tokio::sync::Mutex<Option<Catalog>> = tokio::sync::Mutex::const_new(None);

pub async fn get_catalog(client: &reqwest::Client) -> AppResult<Catalog> {
    {
        let cache = CATALOG_CACHE.lock().await;
        if let Some(cat) = cache.as_ref() {
            if cat.fetched_at.elapsed() < CATALOG_TTL {
                return Ok(Catalog {
                    entries: cat.entries.clone(),
                    fetched_at: cat.fetched_at,
                    source: cat.source.clone(),
                });
            }
        }
    }
    // 简化实现：TTL 过期直接重新拉取（每次调用独立持有）
    fetch_catalog(client).await
}

/// 兼容预检 API URL
const COMPAT_CHECK_URL: &str = "https://dsh.huilinsh.cn/api/compat/check";

/// 兼容预检响应
#[derive(Debug, Deserialize)]
pub(crate) struct CompatCheckResponse {
    #[serde(default)]
    pub compatible: Option<bool>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub conflicts: Vec<CompatConflict>,
    #[serde(default = "default_true")]
    pub has_blocking_conflict: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CompatConflict {
    pub conflict_with: String,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub severity: Option<String>,
}

fn default_true() -> bool { true }

/// 调用官网 /api/compat/check 做安装前兼容预检。
/// 官网不可达时 fail-open（返回 Ok(true)），不阻塞安装。
pub async fn compat_check(
    client: &reqwest::Client,
    plugin_id: &str,
    dsh_ver: &str,
) -> AppResult<(bool, Option<String>, Vec<CompatConflict>)> {
    let url = format!(
        "{}?plugin_id={}&dsh_ver={}",
        COMPAT_CHECK_URL,
        urlencoding::encode(plugin_id),
        urlencoding::encode(dsh_ver)
    );
    let resp = match client.get(&url).timeout(TIMEOUT).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[compat] 官网预检请求失败，跳过: {}", e);
            return Ok((true, None, Vec::new()));
        }
    };
    if !resp.status().is_success() {
        eprintln!("[compat] 官网预检 HTTP {}", resp.status());
        return Ok((true, None, Vec::new()));
    }
    let body: CompatCheckResponse = match resp.json().await {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[compat] 官网预检响应解析失败: {}", e);
            return Ok((true, None, Vec::new()));
        }
    };
    let compatible = body.compatible.unwrap_or(true);
    Ok((compatible, body.note, body.conflicts))
}


/// 验证插件目录签名（Ed25519 + SHA256 预哈希）
/// signature: hex-encoded Ed25519 signature
/// data: the canonical JSON that was signed
pub fn verify_catalog_signature(signature: &str, data: &str, pub_key_bytes: &[u8]) -> bool {
    use ed25519_dalek::VerifyingKey;
    use std::convert::TryFrom;
    
    let sig_bytes = match hex::decode(signature) {
        Ok(b) => b,
        Err(_) => return false,
    };
    
    let sig = match ed25519_dalek::Signature::try_from(sig_bytes.as_slice()) {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    let pub_key = match VerifyingKey::try_from(pub_key_bytes) {
        Ok(k) => k,
        Err(_) => return false,
    };
    
    pub_key.verify_strict(&data.as_bytes(), &sig).is_ok()
}
