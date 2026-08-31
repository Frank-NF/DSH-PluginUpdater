//! DSH 运行时版本探测（V2 §5 / F5）
//!
//! 读配置 plugin_directory 下 package.json 的「dsh」字段：
//! 优先取 dsh.version，其次取 dsh.profile.version，都不存在时返回 None。
//! 版本锚点（min/max_dsh_version 的真实取值）待探测结果回填，见 V2 §2。

use std::fs;
use std::path::Path;

/// 读取指定 profile 目录（含 package.json）中的 DSH 运行时版本
pub fn read_dsh_version(plugin_directory: &str) -> Option<String> {
    let dir = plugin_directory.trim();
    if dir.is_empty() {
        return None;
    }
    read_version_from_package_json(Path::new(dir))
}

fn read_version_from_package_json(dir: &Path) -> Option<String> {
    let pkg_path = dir.join("package.json");
    let content = fs::read_to_string(&pkg_path).ok()?;
    let pkg: serde_json::Value = serde_json::from_str(&content).ok()?;
    let dsh = pkg.get("dsh")?;

    // dsh.version 优先，其次 dsh.profile.version（存在哪个取哪个）
    if let Some(v) = dsh.get("version").and_then(|v| v.as_str()) {
        let v = v.trim();
        if !v.is_empty() {
            return Some(v.to_string());
        }
    }
    if let Some(profile) = dsh.get("profile") {
        if let Some(v) = profile.get("version").and_then(|v| v.as_str()) {
            let v = v.trim();
            if !v.is_empty() {
                return Some(v.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique_temp_dir(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!(
            "dsh_version_probe_{}_{}_{}",
            tag,
            std::process::id(),
            nanos
        ));
        fs::create_dir_all(&dir).expect("创建临时目录失败");
        dir
    }

    #[test]
    fn reads_dsh_version_field() {
        let dir = unique_temp_dir("ver");
        fs::write(
            dir.join("package.json"),
            r#"{"name":"p","dsh":{"version":"3.25.3"}}"#,
        )
        .expect("写入 package.json 失败");
        assert_eq!(
            read_dsh_version(&dir.to_string_lossy()),
            Some("3.25.3".to_string())
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn falls_back_to_profile_version() {
        let dir = unique_temp_dir("profile");
        fs::write(
            dir.join("package.json"),
            r#"{"dsh":{"profile":{"version":"1.2.3"}}}"#,
        )
        .expect("写入 package.json 失败");
        assert_eq!(
            read_dsh_version(&dir.to_string_lossy()),
            Some("1.2.3".to_string())
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn returns_none_when_missing() {
        let dir = unique_temp_dir("none");
        // 无 package.json
        assert_eq!(read_dsh_version(&dir.to_string_lossy()), None);
        // 有 package.json 但无 dsh 字段
        fs::write(dir.join("package.json"), r#"{"name":"p"}"#)
            .expect("写入 package.json 失败");
        assert_eq!(read_dsh_version(&dir.to_string_lossy()), None);
        // 有 dsh 字段但无 version
        fs::write(dir.join("package.json"), r#"{"dsh":{"x":1}}"#)
            .expect("写入 package.json 失败");
        assert_eq!(read_dsh_version(&dir.to_string_lossy()), None);
        // 空目录参数
        assert_eq!(read_dsh_version(""), None);
        let _ = fs::remove_dir_all(&dir);
    }
}