use crate::error::{AppError, AppResult};
use serde::Deserialize;
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
const CATALOG_TTL: Duration = Duration::from_secs(600);

/// 官方插件目录的一条记录
#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
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

/// 已加载的目录（含获取时间，用于 TTL 缓存）
pub struct Catalog {
    pub entries: Vec<CatalogEntry>,
    pub fetched_at: Instant,
    pub source: String,
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

/// 拉取官方插件目录。
/// 源顺序：npm 镜像包（腾讯镜像 → npmjs）→ 官方 GitHub Pages 直连。
pub async fn fetch_catalog(client: &reqwest::Client) -> AppResult<Catalog> {
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

    Err(AppError::Other(format!(
        "所有目录源均不可达，最后错误: {}",
        last_err.unwrap_or_default()
    )))
}

/// npm registry 查询包的最新版本（无 API 配额限制）

/// 查询 npm 包最新版本及其 tarball 下载地址（腾讯镜像优先）
pub async fn npm_latest_meta(
    client: &reqwest::Client,
    npm_name: &str,
) -> AppResult<(String, Option<String>)> {
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
                            return Ok((v.to_string(), tarball));
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

