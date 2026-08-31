use crate::error::{AppError, AppResult, PluginInfo, PluginManifest};
use crate::manifest::{create_agent_core_manifest, manifest_exists, read_manifest};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_plugin_directory(root_dir: &str) -> AppResult<Vec<PluginInfo>> {
    let root = Path::new(root_dir);
    if !root.exists() || !root.is_dir() {
        return Err(AppError::DirectoryNotFound(root_dir.to_string()));
    }

    let mut plugins: Vec<PluginInfo> = Vec::new();

    // 如果是 DSH profile 目录（cordis 结构），按 profile 方式扫描
    if is_dsh_profile(root) {
        plugins.extend(scan_dsh_profile(root));
        sort_plugins(&mut plugins);
        return Ok(plugins);
    }

    // 检查根目录是否是 Agent 本体目录
    if is_agent_core_directory(root) {
        if let Ok(core_manifest) = create_agent_core_manifest(root) {
            plugins.push(PluginInfo {
                manifest: core_manifest,
                install_path: root.to_string_lossy().to_string(),
                latest_version: None,
                release_url: None,
                download_url: None,
                release_notes: None,
                update_available: false,
                check_error: None,
                description_zh: None,
                description_en: None,
                category: None,
                stars: None,
                downloads: None,
            sha256: None,
            });
        }
    }

    // 扫描一级子目录
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // 子目录若是 DSH profile（如 plugin-sources 本身是目录），先按普通插件扫描
                if let Some(plugin_info) = scan_single_plugin(&path) {
                    plugins.push(plugin_info);
                }
            }
        }
    }

    sort_plugins(&mut plugins);
    Ok(plugins)
}

fn sort_plugins(plugins: &mut Vec<PluginInfo>) {
    // 按类型排序：agent-core 在前，然后按名称
    plugins.sort_by(|a, b| {
        let type_order = |t: &str| if t == "agent-core" { 0 } else { 1 };
        type_order(&a.manifest.r#type)
            .cmp(&type_order(&b.manifest.r#type))
            .then_with(|| a.manifest.name.cmp(&b.manifest.name))
    });
}

/// 判断目录是否是 DSH profile（cordis 结构）：含 cordis.yml，或含 package.json 且带 node_modules
pub fn is_dsh_profile(dir: &Path) -> bool {
    if dir.join("cordis.yml").exists() {
        return true;
    }
    // package.json 带 dsh.profile 字段
    let pkg_path = dir.join("package.json");
    if pkg_path.exists() {
        if let Ok(content) = fs::read_to_string(&pkg_path) {
            if let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&content) {
                if pkg.get("dsh").is_some() {
                    return true;
                }
            }
        }
    }
    // node_modules 下存在 dsh- 插件
    let nm_dir = dir.join("node_modules");
    if nm_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&nm_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("dsh-") {
                    return true;
                }
            }
        }
    }
    false
}

/// 扫描 DSH profile 目录：node_modules 下的 dsh-* 插件 + plugin-sources 下的本地插件
pub fn scan_dsh_profile(dir: &Path) -> Vec<PluginInfo> {
    let mut plugins = Vec::new();

    // 1. 扫描 node_modules 下的插件（含 @scope/dsh-xxx）
    let nm_dir = dir.join("node_modules");
    if nm_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&nm_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with('@') {
                    // scoped 包：@scope/dsh-xxx
                    if let Ok(scoped) = fs::read_dir(&path) {
                        for se in scoped.flatten() {
                            if se.path().is_dir() {
                                if let Some(p) = scan_cordis_plugin(&se.path()) {
                                    plugins.push(p);
                                }
                            }
                        }
                    }
                } else if name.starts_with("dsh-") {
                    if let Some(p) = scan_cordis_plugin(&path) {
                        plugins.push(p);
                    }
                }
            }
        }
    }

    // 2. 扫描 plugin-sources 下的本地插件
    let ps_dir = dir.join("plugin-sources");
    if ps_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&ps_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(p) = scan_cordis_plugin(&path) {
                        plugins.push(p);
                    }
                }
            }
        }
    }

    // 3. 顶层目录中的本地插件（如 .dsh/profiles/web 直接放插件目录的情况）
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                // 跳过已知的系统目录
                if ["node_modules", "plugin-sources", ".dsh-market", "tmp", ".pnpm-store", ".git"].contains(&name.as_str()) {
                    continue;
                }
                if let Some(p) = scan_cordis_plugin(&path) {
                    plugins.push(p);
                }
            }
        }
    }

    // 去重：pnpm 别名安装可能产生同名插件（如 dsh-balance 与 @scope/dsh-balance）
    let mut seen = std::collections::HashSet::new();
    plugins.retain(|p| seen.insert(p.manifest.id.clone()));

    plugins
}

/// 从 cordis 插件的 package.json 读取信息，生成 PluginInfo
/// 该插件目录是否被 profile 根 package.json 声明（dependencies 或 dsh.profile.bundles）
/// —— 覆盖不叫 dsh-* 的插件（如 @liustack/modlens）
fn is_declared_plugin(dir: &Path) -> bool {
    // dir = .../node_modules/<pkg> 或 .../node_modules/@scope/<pkg>
    // 从 dir 向上找最近的含 package.json 的目录（即 profile 根）
    let mut cur = dir;
    let mut profile_root: Option<&Path> = None;
    for _ in 0..4 {
        match cur.parent() {
            Some(p) => cur = p,
            None => return false,
        }
        if cur.join("package.json").is_file() {
            profile_root = Some(cur);
            break;
        }
    }
    let Some(profile) = profile_root else {
        return false;
    };
    let Ok(content) = fs::read_to_string(profile.join("package.json")) else {
        return false;
    };
    let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&content) else {
        return false;
    };

    // 完整包名：scoped 用 @scope/name
    let parent_name = dir
        .parent()
        .and_then(|p| p.file_name())
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default();
    let dir_name = dir
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default();
    let full_name = if parent_name.starts_with('@') {
        format!("{}/{}", parent_name, dir_name)
    } else {
        dir_name
    };

    if pkg["dependencies"]
        .as_object()
        .map(|d| d.contains_key(&full_name))
        .unwrap_or(false)
    {
        return true;
    }
    if pkg["dsh"]["profile"]["bundles"]
        .as_array()
        .map(|b| b.iter().any(|x| x.as_str() == Some(&full_name)))
        .unwrap_or(false)
    {
        return true;
    }
    false
}

fn scan_cordis_plugin(dir: &Path) -> Option<PluginInfo> {
    let pkg_path = dir.join("package.json");
    if !pkg_path.exists() {
        return None;
    }
    let content = fs::read_to_string(&pkg_path).ok()?;
    let pkg: serde_json::Value = serde_json::from_str(&content).ok()?;

    let name = pkg.get("name")?.as_str()?.to_string();

    // 识别 DSH 插件：目录名以 dsh- 开头（含 @scope/dsh-*），或 keywords 明确声明 dsh-plugin /
    // deepseek-harness-plugin，或被 profile 根 dependencies/bundles 声明。
    // 注意：裸 deepseek-harness 关键词匹配曾把官方依赖包（cordis/minato 等普遍打此
    // keyword）整树误收进插件列表（真机 238 个），必须限定到 -plugin 后缀。
    let keywords = pkg.get("keywords").and_then(|k| k.as_array());
    let is_dsh_plugin = name.starts_with("dsh-")
        || name.contains("/dsh-")
        || keywords.map_or(false, |ks| ks.iter().any(|k| {
            k.as_str().map_or(false, |s| {
                s == "dsh-plugin" || s == "deepseek-harness-plugin"
            })
        }))
        || is_declared_plugin(dir);
    if !is_dsh_plugin {
        return None;
    }

    let version = pkg.get("version").and_then(|v| v.as_str()).unwrap_or("0.0.0").to_string();
    // 系统内置（DSH 本体/官方运行时组件）：@deepseek-ai/* scope 与内置市场 dshmarket。
    // 标记为 agent-core：前端统计与更新列表自动排除（与 DSH Agent 本体同类型）。
    let is_builtin = name.starts_with("@deepseek-ai/") || name == "dshmarket";
    let plugin_type = if is_builtin { "agent-core" } else { "plugin" };
    let description = pkg.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string();
    let author = pkg.get("author").and_then(|a| a.as_str())
        .or_else(|| pkg.get("author").and_then(|a| a.get("name")).and_then(|n| n.as_str()))
        .unwrap_or("").to_string();
    let homepage = pkg.get("homepage").and_then(|h| h.as_str()).unwrap_or("").to_string();
    let repo_url = pkg.get("repository").and_then(|r| r.as_str())
        .or_else(|| pkg.get("repository").and_then(|r| r.get("url")).and_then(|u| u.as_str()))
        .unwrap_or("").to_string();
    let github_repo = extract_github_repo(&repo_url);

    Some(PluginInfo {
        manifest: PluginManifest {
            id: name.clone(),
            name: name.clone(),
            description,
            github_repo,
            current_version: version,
            enabled: true,
            r#type: plugin_type.to_string(),
            author,
            homepage,
        },
        install_path: dir.to_string_lossy().to_string(),
        latest_version: None,
        release_url: None,
        download_url: None,
        release_notes: None,
        update_available: false,
        check_error: None,
        description_zh: None,
        description_en: None,
        category: None,
        stars: None,
        downloads: None,
    sha256: None,
    })
}

/// 从 repository URL 提取 "owner/repo"
fn extract_github_repo(url: &str) -> String {
    let clean = url.trim_start_matches("git+").trim_start_matches("git://");
    if let Some(rest) = clean.strip_prefix("https://github.com/") {
        return rest.trim_end_matches(".git").to_string();
    }
    if let Some(rest) = clean.strip_prefix("ssh://git@github.com/") {
        return rest.trim_end_matches(".git").to_string();
    }
    if let Some(rest) = clean.strip_prefix("git@github.com:") {
        return rest.trim_end_matches(".git").to_string();
    }
    if let Some(rest) = clean.strip_prefix("github:") {
        return rest.to_string();
    }
    String::new()
}

fn scan_single_plugin(dir: &Path) -> Option<PluginInfo> {
    if !manifest_exists(dir) {
        // 兼容 cordis 插件：有 package.json 的 dsh-* 目录
        if let Some(p) = scan_cordis_plugin(dir) {
            return Some(p);
        }
        // 递归检查是否有嵌套的插件目录
        return scan_nested_plugin(dir);
    }

    match read_manifest(dir) {
        Ok(manifest) => Some(PluginInfo {
            manifest,
            install_path: dir.to_string_lossy().to_string(),
            latest_version: None,
            release_url: None,
            download_url: None,
            release_notes: None,
            update_available: false,
            check_error: None,
            description_zh: None,
            description_en: None,
            category: None,
            stars: None,
            downloads: None,
        sha256: None,
        }),
        Err(_) => None,
    }
}

fn scan_nested_plugin(dir: &Path) -> Option<PluginInfo> {
    // 只深入一层查找
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && manifest_exists(&path) {
                if let Ok(manifest) = read_manifest(&path) {
                    return Some(PluginInfo {
                        manifest,
                        install_path: path.to_string_lossy().to_string(),
                        latest_version: None,
                        release_url: None,
                        download_url: None,
                        release_notes: None,
                        update_available: false,
                        check_error: None,
                        description_zh: None,
                        description_en: None,
                        category: None,
                        stars: None,
                        downloads: None,
                    sha256: None,
                    });
                }
            }
            // 兼容 cordis 插件
            if path.is_dir() {
                if let Some(p) = scan_cordis_plugin(&path) {
                    return Some(p);
                }
            }
        }
    }
    None
}

fn is_agent_core_directory(dir: &Path) -> bool {
    // 判断是否是 Agent 本体目录：包含 VERSION 文件或 dsh-agent 可执行文件
    dir.join("VERSION").exists()
        || dir.join("dsh-agent.exe").exists()
        || dir.join("dsh-agent").exists()
}

pub fn find_plugin_by_id<'a>(plugins: &'a [PluginInfo], id: &str) -> Option<&'a PluginInfo> {
    plugins.iter().find(|p| p.manifest.id == id)
}

pub fn validate_plugin_directory(path: &str) -> Result<(), String> {
    let p = Path::new(path);
    if !p.exists() {
        return Err("目录不存在".to_string());
    }
    if !p.is_dir() {
        return Err("路径不是目录".to_string());
    }
    Ok(())
}

pub fn get_directory_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}
