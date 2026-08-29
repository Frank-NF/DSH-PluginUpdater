use crate::error::{AppError, AppResult, PluginManifest};
use std::fs;
use std::path::Path;

pub const MANIFEST_FILENAME: &str = "plugin.manifest.json";

pub fn read_manifest(plugin_dir: &Path) -> AppResult<PluginManifest> {
    let manifest_path = plugin_dir.join(MANIFEST_FILENAME);
    if !manifest_path.exists() {
        return Err(AppError::ManifestNotFound(
            plugin_dir.to_string_lossy().to_string(),
        ));
    }
    let content = fs::read_to_string(&manifest_path)?;
    let manifest: PluginManifest = serde_json::from_str(&content)
        .map_err(|e| AppError::ManifestInvalid(format!("{}: {}", manifest_path.display(), e)))?;
    Ok(manifest)
}

pub fn write_manifest(plugin_dir: &Path, manifest: &PluginManifest) -> AppResult<()> {
    let manifest_path = plugin_dir.join(MANIFEST_FILENAME);
    let content = serde_json::to_string_pretty(manifest)?;
    fs::write(&manifest_path, content)?;
    Ok(())
}

pub fn manifest_exists(plugin_dir: &Path) -> bool {
    plugin_dir.join(MANIFEST_FILENAME).exists()
}

pub fn create_agent_core_manifest(plugin_dir: &Path) -> AppResult<PluginManifest> {
    let version_file = plugin_dir.join("VERSION");
    let current_version = if version_file.exists() {
        fs::read_to_string(&version_file).unwrap_or_default().trim().to_string()
    } else {
        "0.0.0".to_string()
    };

    Ok(PluginManifest {
        id: "dsh-agent-core".to_string(),
        name: "DSH Agent 本体".to_string(),
        description: "DSH Agent 核心程序，提供基础运行环境与能力调度".to_string(),
        github_repo: "DSH-Team/DSH-Agent".to_string(),
        current_version,
        enabled: true,
        r#type: "agent-core".to_string(),
        author: "DSH Team".to_string(),
        homepage: "https://dsh-update.hk".to_string(),
    })
}
