use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON解析错误: {0}")]
    Json(#[from] serde_json::Error),
    #[error("网络请求错误: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Zip解压错误: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("版本解析错误: {0}")]
    Semver(#[from] semver::Error),
    #[error("插件目录不存在: {0}")]
    DirectoryNotFound(String),
    #[error("插件manifest缺失: {0}")]
    ManifestNotFound(String),
    #[error("插件manifest格式错误: {0}")]
    ManifestInvalid(String),
    #[error("操作被取消")]
    Cancelled,
    #[error("兼容性预检失败: {0}")]
    CompatCheck(String),
    #[error("存在阻塞级冲突: {0}")]
    BlockingConflict(String),
    #[error("自我更新失败: {0}")]
    SelfUpdate(String),
    #[error("其他错误: {0}")]
    Other(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub github_repo: String,
    #[serde(default)]
    pub current_version: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_type")]
    pub r#type: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub homepage: String,
}

fn default_enabled() -> bool {
    true
}

fn default_type() -> String {
    "plugin".to_string()
}

/// 市场插件（官方目录精简条目，供插件市场浏览）
/// 环境检查项
#[derive(Debug, Clone, Serialize)]
pub struct EnvCheckItem {
    pub id: String,
    pub name: String,
    pub status: String, // ok | warn | error
    pub message: String,
    #[serde(default)]
    pub fix_hint: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MarketPlugin {
    pub name: String,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub stars: Option<u64>,
    #[serde(default)]
    pub downloads: Option<u64>,
    #[serde(default)]
    pub desc_zh: Option<String>,
    #[serde(default)]
    pub desc_en: Option<String>,
    #[serde(default)]
    pub npm: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub manifest: PluginManifest,
    pub install_path: String,
    pub latest_version: Option<String>,
    pub release_url: Option<String>,
    pub download_url: Option<String>,
    pub release_notes: Option<String>,
    pub update_available: bool,
    pub check_error: Option<String>,
    #[serde(default)]
    pub description_zh: Option<String>,
    #[serde(default)]
    pub description_en: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub stars: Option<u64>,
    #[serde(default)]
    pub downloads: Option<u64>,
    #[serde(default)]
    pub sha256: Option<String>,
    /// 本体预装（profile 根 package.json dependencies/bundles 声明）：
    /// npm 上游更新由 DSH 本体统一管理，工具不提示更新、不计入可更新数
    #[serde(default)]
    pub bundled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub html_url: String,
    pub body: String,
    pub assets: Vec<GitHubAsset>,
    pub published_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProgress {
    pub plugin_id: String,
    pub phase: String,
    pub percent: u8,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub proxy_base_url: String,
    pub plugin_directory: String,
    pub auto_check_updates: bool,
    pub backup_before_update: bool,
    #[serde(default)]
    pub install_registry: String,
    #[serde(default)]
    pub server_host: String,
    #[serde(default = "default_server_port")]
    pub server_port: u16,
    #[serde(default)]
    pub server_user: String,
    #[serde(default)]
    pub server_key: String,
    #[serde(default)]
    pub server_remote_dir: String,
    #[serde(default)]
    pub server_dsh_dir: String,
    #[serde(default)]
    pub server_update_cmd: String,
}

fn default_server_port() -> u16 {
    22
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            proxy_base_url: String::new(),
            plugin_directory: String::new(),
            auto_check_updates: true,
            backup_before_update: true,
          install_registry: String::new(),
            server_host: String::new(),
            server_port: 22,
            server_user: String::new(),
            server_key: String::new(),
            server_remote_dir: String::new(),
            server_dsh_dir: String::new(),
            server_update_cmd: String::new(),
        }
    }
}
