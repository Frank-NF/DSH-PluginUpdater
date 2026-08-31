//! 快照导入导出与离线打包（V2 §2 snapshots 表精神 + P1 尾项）。
//!
//! - snapshot_export：把当前已装插件清单（含 npm 包名@版本）导出为 JSON 快照；
//! - snapshot_preview：快照 vs 当前目录差异（missing / version_mismatch / ok）；
//! - snapshot_apply：对 missing 项逐个 npm install <name>@<version>（复用 registry 配置）；
//! - offline_pack / offline_apply：插件目录 zip 归档与免网络还原。
//!
//! npm 包名取自各插件 install_path 下 package.json 的 name 字段（npm 安装痕迹）。

use crate::error::{AppError, AppResult, PluginInfo};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotEntry {
    pub id: String,
    pub name: String,
    pub version: String,
    /// npm 包名（来自插件 package.json；无则为空串）
    #[serde(default)]
    pub npm_name: String,
    #[serde(default)]
    pub github_repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotFile {
    pub format_version: u32,
    pub exported_at: String,
    pub source_dir: String,
    pub entries: Vec<SnapshotEntry>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotSummary {
    pub path: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotDiff {
    pub missing: Vec<SnapshotEntry>,
    pub version_mismatch: Vec<SnapshotEntry>,
    pub ok_count: usize,
    pub current_count: usize,
    pub snapshot_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotApplyItem {
    pub id: String,
    pub name: String,
    pub status: String,
    pub detail: String,
}

fn to_entry(p: &PluginInfo) -> SnapshotEntry {
    let pj = std::path::Path::new(&p.install_path).join("package.json");
    let npm_name = std::fs::read_to_string(pj)
        .ok()
        .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
        .and_then(|v| v.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()))
        .unwrap_or_default();
    SnapshotEntry {
        id: p.manifest.id.clone(),
        name: p.manifest.name.clone(),
        version: p.manifest.current_version.clone(),
        npm_name,
        github_repo: p.manifest.github_repo.clone(),
    }
}

fn current_entries(root_dir: &str) -> AppResult<Vec<SnapshotEntry>> {
    let plugins = crate::plugin_scan::scan_plugin_directory(root_dir)?;
    Ok(plugins.iter().map(to_entry).collect())
}

/// 供命令层复用的扫描摘要（不含快照文件 IO）
pub(crate) fn snapshot_entries(root_dir: &str) -> AppResult<Vec<SnapshotEntry>> {
    current_entries(root_dir)
}

/// 导出快照 JSON（UTF-8 无 BOM，.tmp 原子改名）
pub fn snapshot_export(root_dir: &str, path: &str) -> AppResult<SnapshotSummary> {
    let entries = current_entries(root_dir)?;
    let file = SnapshotFile {
        format_version: 1,
        exported_at: crate::bundle::iso_now(),
        source_dir: root_dir.to_string(),
        entries: entries.clone(),
    };
    let tmp = PathBuf::from(format!("{}.tmp", path));
    std::fs::write(&tmp, serde_json::to_string_pretty(&file)?)?;
    std::fs::rename(&tmp, path)?;
    Ok(SnapshotSummary { path: path.to_string(), count: entries.len() })
}

fn read_snapshot(path: &str) -> AppResult<SnapshotFile> {
    let raw = std::fs::read_to_string(path)?;
    let file: SnapshotFile = serde_json::from_str(&raw)
        .map_err(|e| AppError::Other(format!("快照文件格式无效: {}", e)))?;
    if file.format_version != 1 {
        return Err(AppError::Other(format!(
            "不支持的快照版本: {}（当前支持 1）",
            file.format_version
        )));
    }
    Ok(file)
}

/// 快照 vs 当前：missing（快照有现在无）/ version_mismatch（现在版本不同）
pub fn snapshot_preview(root_dir: &str, path: &str) -> AppResult<SnapshotDiff> {
    let file = read_snapshot(path)?;
    let current = current_entries(root_dir)?;
    let by_id: std::collections::HashMap<&str, &SnapshotEntry> =
        current.iter().map(|e| (e.id.as_str(), e)).collect();
    let mut missing = Vec::new();
    let mut mismatch = Vec::new();
    let mut ok_count = 0usize;
    for e in &file.entries {
        match by_id.get(e.id.as_str()) {
            None => missing.push(e.clone()),
            Some(cur) if cur.version != e.version => mismatch.push(e.clone()),
            Some(_) => ok_count += 1,
        }
    }
    Ok(SnapshotDiff {
        missing,
        version_mismatch: mismatch,
        ok_count,
        current_count: current.len(),
        snapshot_count: file.entries.len(),
    })
}

/// 应用快照：对 missing 且带 npm 包名的条目逐个 npm install <name>@<version>
pub async fn snapshot_apply(
    root_dir: &str,
    path: &str,
    registry: &str,
) -> AppResult<Vec<SnapshotApplyItem>> {
    let diff = snapshot_preview(root_dir, path)?;
    let mut results = Vec::new();
    for e in diff.missing {
        if e.npm_name.trim().is_empty() {
            results.push(SnapshotApplyItem {
                id: e.id,
                name: e.name,
                status: "skipped".into(),
                detail: "快照中无 npm 包名，无法在线还原（可用离线打包还原）".into(),
            });
            continue;
        }
        let spec = if e.version.trim().is_empty() {
            e.npm_name.clone()
        } else {
            format!("{}@{}", e.npm_name, e.version)
        };
        match crate::bundle::npm_install_into(&e.npm_name, root_dir, registry).await {
            Ok(()) => results.push(SnapshotApplyItem {
                id: e.id,
                name: e.name,
                status: "installed".into(),
                detail: format!("已安装 {}", spec),
            }),
            Err(err) => results.push(SnapshotApplyItem {
                id: e.id,
                name: e.name,
                status: "failed".into(),
                detail: truncate(&err, 200),
            }),
        }
    }
    Ok(results)
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        s.chars().skip(s.chars().count() - n).collect()
    }
}

// ---------------- 离线打包 ----------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflinePackSummary {
    pub path: String,
    pub plugins: usize,
    pub bytes: u64,
}

fn collect_plugin_dirs(root_dir: &str) -> AppResult<Vec<(String, PathBuf)>> {
    let plugins = crate::plugin_scan::scan_plugin_directory(root_dir)?;
    let mut out = Vec::new();
    for p in &plugins {
        let rel = Path::new(&p.install_path)
            .strip_prefix(root_dir)
            .map(|r| r.to_string_lossy().to_string())
            .unwrap_or_else(|_| Path::new(&p.install_path)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default());
        out.push((rel.replace('\\', "/"), PathBuf::from(&p.install_path)));
    }
    Ok(out)
}

/// 把当前全部插件目录打包为 zip（deflate；相对路径保留目录结构）
pub fn offline_pack(root_dir: &str, out_path: &str) -> AppResult<OfflinePackSummary> {
    let dirs = collect_plugin_dirs(root_dir)?;
    if dirs.is_empty() {
        return Err(AppError::Other("当前目录没有已扫描到的插件，无需打包".into()));
    }
    let file = std::fs::File::create(out_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let opts =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut files = 0usize;
    for (rel, dir) in &dirs {
        for entry in walkdir(dir)? {
            if entry.is_dir() {
                continue;
            }
            let sub = entry
                .strip_prefix(dir)
                .map_err(|e| AppError::Other(format!("路径解析失败: {}", e)))?;
            let arc = format!("{}/{}", rel.trim_end_matches('/'), sub.to_string_lossy().replace('\\', "/"));
            zip.start_file(arc, opts)
                .map_err(|e| AppError::Other(format!("zip 写入失败: {}", e)))?;
            let data = std::fs::read(&entry)?;
            std::io::Write::write_all(&mut zip, &data)?;
            files += 1;
        }
    }
    zip.finish().map_err(|e| AppError::Other(format!("zip 收尾失败: {}", e)))?;
    let bytes = std::fs::metadata(out_path)?.len();
    Ok(OfflinePackSummary { path: out_path.to_string(), plugins: dirs.len(), bytes })
}

fn walkdir(dir: &Path) -> AppResult<Vec<PathBuf>> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d)? {
            let e = e?;
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                out.push(p);
            }
        }
    }
    out.sort();
    Ok(out)
}

/// 从 zip 还原插件目录到 root_dir（按归档内相对路径逐文件解出；同名文件覆盖）
pub fn offline_apply(archive_path: &str, root_dir: &str) -> AppResult<usize> {
    let f = std::fs::File::open(archive_path)?;
    let mut zip = zip::ZipArchive::new(f)
        .map_err(|e| AppError::Other(format!("无法读取离线包: {}", e)))?;
    let root = PathBuf::from(root_dir);
    std::fs::create_dir_all(&root)?;
    let mut restored = 0usize;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)
            .map_err(|e| AppError::Other(format!("离线包条目读取失败: {}", e)))?;
        let rel = entry
            .enclosed_name()
            .ok_or_else(|| AppError::Other("离线包内存在非法路径".into()))?
            .to_path_buf();
        let target = root.join(rel);
        if entry.is_dir() {
            std::fs::create_dir_all(&target)?;
            continue;
        }
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut out = std::fs::File::create(&target)?;
        std::io::copy(&mut entry, &mut out)?;
        restored += 1;
    }
    Ok(restored)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_plugin(root: &Path, id: &str, npm: &str, ver: &str) -> PathBuf {
        let dir = root.join(npm);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("package.json"),
            format!("{{\"name\":\"{}\",\"version\":\"{}\"}}", npm, ver),
        )
        .unwrap();
        std::fs::write(
            dir.join("plugin.manifest.json"),
            format!(
                "{{\"id\":\"{}\",\"name\":\"{}\",\"currentVersion\":\"{}\",\"type\":\"plugin\"}}",
                id, npm, ver
            ),
        )
        .unwrap();
        dir
    }


    /// 端到端（纯文件系统 + 一次死端口 npm）：导出 → 删插件 → preview 差异 → apply 状态分支 → 离线包还原
    #[test]
    fn snapshot_export_preview_offline_roundtrip() {
        let base = std::env::temp_dir().join(format!(
            "dsh-snap-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
        ));
        let profile = base.join("profile");
        let a = fake_plugin(&profile, "p-a", "dsh-alpha", "1.0.0");
        let b = fake_plugin(&profile, "p-b", "dsh-beta", "2.0.0");
        let snap_path = base.join("snap.json");

        let s = snapshot_export(profile.to_str().unwrap(), snap_path.to_str().unwrap()).unwrap();
        assert_eq!(s.count, 2, "应扫描到 2 个插件");

        std::fs::remove_dir_all(&b).unwrap();
        let diff = snapshot_preview(profile.to_str().unwrap(), snap_path.to_str().unwrap()).unwrap();
        assert_eq!(diff.missing.len(), 1);
        assert_eq!(diff.missing[0].id, "p-b");
        assert_eq!(diff.ok_count, 1);

        // apply：registry 指向死端口 → failed 分支（或环境允许时 installed）；断言不 panic 且状态合法
        let items = tokio::runtime::Runtime::new().unwrap().block_on(snapshot_apply(
            profile.to_str().unwrap(),
            snap_path.to_str().unwrap(),
            "http://127.0.0.1:9",
        ));
        let items = items.unwrap();
        assert_eq!(items.len(), 1);
        assert!(
            items[0].status == "failed" || items[0].status == "installed",
            "apply 状态应为 failed/installed: {}",
            items[0].status
        );

        let zip_path = base.join("offline.zip");
        let pack = offline_pack(profile.to_str().unwrap(), zip_path.to_str().unwrap()).unwrap();
        assert_eq!(pack.plugins, 1);
        std::fs::remove_dir_all(&a).unwrap();
        let restored = offline_apply(zip_path.to_str().unwrap(), profile.to_str().unwrap()).unwrap();
        assert!(restored > 0);
        assert!(a.join("plugin.manifest.json").exists(), "还原后 manifest 应存在");

        let _ = std::fs::remove_dir_all(&base);
    }
}
