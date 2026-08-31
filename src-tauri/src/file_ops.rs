use crate::error::{AppError, AppResult, PluginManifest};
use std::io::Read;
use crate::manifest::write_manifest;
use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use flate2::read::GzDecoder;
use tar::Archive as TarArchive;

pub struct PluginFileManager {
    backup_root: PathBuf,
}

impl PluginFileManager {
    pub fn new(plugin_dir: &str) -> Self {
        let backup_root = Path::new(plugin_dir).join(".updater_backups");
        PluginFileManager { backup_root }
    }

    pub fn ensure_backup_dir(&self) -> AppResult<()> {
        if !self.backup_root.exists() {
            fs::create_dir_all(&self.backup_root)?;
        }
        Ok(())
    }

    pub fn backup_plugin(&self, plugin_path: &str, plugin_id: &str) -> AppResult<String> {
        self.ensure_backup_dir()?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let backup_name = format!("{}_{}", plugin_id, timestamp);
        let backup_path = self.backup_root.join(&backup_name);

        let src = Path::new(plugin_path);
        if !src.exists() {
            return Err(AppError::DirectoryNotFound(plugin_path.to_string()));
        }

        copy_directory_recursive(src, &backup_path)?;
        Ok(backup_path.to_string_lossy().to_string())
    }

    pub fn restore_backup(&self, backup_path: &str, target_path: &str) -> AppResult<()> {
        let backup = Path::new(backup_path);
        if !backup.exists() {
            return Err(AppError::DirectoryNotFound(backup_path.to_string()));
        }

        let target = Path::new(target_path);
        if target.exists() {
            fs::remove_dir_all(target)?;
        }

        copy_directory_recursive(backup, target)?;
        Ok(())
    }

    pub fn uninstall_plugin(&self, plugin_path: &str, plugin_id: &str) -> AppResult<()> {
        // Backup before uninstall
        let _backup = self.backup_plugin(plugin_path, plugin_id)?;

        let path = Path::new(plugin_path);
        if !path.exists() {
            return Err(AppError::DirectoryNotFound(plugin_path.to_string()));
        }

        fs::remove_dir_all(path)?;
        Ok(())
    }

    pub fn set_plugin_enabled(
        &self,
        plugin_path: &str,
        manifest: &mut PluginManifest,
        enabled: bool,
    ) -> AppResult<()> {
        manifest.enabled = enabled;
        let path = Path::new(plugin_path);
        write_manifest(path, manifest)?;
        Ok(())
    }

    pub fn extract_update_package(
        &self,
        zip_path: &str,
        target_path: &str,
    ) -> AppResult<()> {
        // npm 包是 .tgz（gzip+tar），GitHub release 是 .zip —— 按扩展名分流
        if zip_path.ends_with(".tgz") || zip_path.ends_with(".tar.gz") {
            return self.extract_tgz(zip_path, target_path);
        }

        let file = fs::File::open(zip_path)?;
        let mut archive = ZipArchive::new(file)?;

        let target = Path::new(target_path);

        for i in 0..archive.len() {
            let mut file_in_zip = archive.by_index(i)?;
            let outpath = match file_in_zip.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            let outpath = strip_top_level_dir(&outpath);
            // 安全化（F3）：strip 后仍可能残留 .. / 根 / 盘符前缀组件，统一拒绝路径穿越条目
            let outpath = match sanitize_rel_path(&outpath) {
                Some(p) => p,
                None => {
                    log::warn!("[zip] 拒绝可疑路径条目并跳过: {}", file_in_zip.name());
                    continue;
                }
            };
            let full_outpath = target.join(&outpath);

            if file_in_zip.name().ends_with('/') {
                fs::create_dir_all(&full_outpath)?;
            } else {
                if let Some(p) = full_outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = fs::File::create(&full_outpath)?;
                std::io::copy(&mut file_in_zip, &mut outfile)?;
            }
        }

        Ok(())
    }

    /// 解压 npm .tgz 包（gzip + tar，去掉顶层 package/ 目录）
    fn extract_tgz(&self, tgz_path: &str, target_path: &str) -> AppResult<()> {
        let file = fs::File::open(tgz_path)?;
        let gz = GzDecoder::new(file);
        let mut archive = TarArchive::new(gz);
        let target = Path::new(target_path);

        for entry in archive.entries()? {
            let mut entry = entry?;
            let raw = entry.path()?.to_path_buf();
            let outpath = strip_top_level_dir(&raw);
            // 安全化（F3）：目录项此前未做任何校验（真实缺口），strip 后统一过白名单
            let outpath = match sanitize_rel_path(&outpath) {
                Some(p) => p,
                None => {
                    log::warn!("[tgz] 拒绝可疑路径条目并跳过: {}", raw.display());
                    continue;
                }
            };
            let full_outpath = target.join(&outpath);

            if entry.header().entry_type().is_dir() {
                fs::create_dir_all(&full_outpath)?;
            } else {
                if let Some(p) = full_outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                entry.unpack(&full_outpath)?;
            }
        }

        Ok(())
    }

    pub fn update_plugin_version(
        &self,
        plugin_path: &str,
        new_version: &str,
    ) -> AppResult<()> {
        use crate::manifest::read_manifest;
        let path = Path::new(plugin_path);

        // DSH 原生格式：plugin.manifest.json
        if path.join(crate::manifest::MANIFEST_FILENAME).exists() {
            let mut manifest = read_manifest(path)?;
            manifest.current_version = new_version.to_string();
            write_manifest(path, &manifest)?;
            return Ok(());
        }

        // cordis/npm 格式：package.json 的 version 字段
        let pkg_path = path.join("package.json");
        if !pkg_path.exists() {
            return Err(AppError::ManifestNotFound(
                plugin_path.to_string(),
            ));
        }
        let content = fs::read_to_string(&pkg_path)?;
        let mut pkg: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| AppError::ManifestInvalid(format!("{}: {}", pkg_path.display(), e)))?;
        if !pkg.is_object() {
            return Err(AppError::ManifestInvalid(format!("{}: 不是合法 JSON 对象", pkg_path.display())));
        }
        pkg["version"] = serde_json::Value::String(new_version.to_string());
        fs::write(&pkg_path, serde_json::to_string_pretty(&pkg)?)?;
        Ok(())
    }

    pub fn list_backups(&self) -> AppResult<Vec<BackupInfo>> {
        self.ensure_backup_dir()?;
        let mut backups = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.backup_root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = match path.file_name() {
                        Some(n) => n.to_string_lossy().to_string(),
                        None => continue, // Skip entries with invalid names
                    };
                    let metadata = fs::metadata(&path)?;
                    backups.push(BackupInfo {
                        name,
                        path: path.to_string_lossy().to_string(),
                        size: get_dir_size(&path),
                        created: metadata
                            .created()
                            .map(|t| format!("{:?}", t))
                            .unwrap_or_default(),
                    });
                }
            }
        }

        backups.sort_by(|a, b| b.name.cmp(&a.name));
        Ok(backups)
    }

    pub fn cleanup_old_backups(&self, keep_count: usize) -> AppResult<usize> {
        let backups = self.list_backups()?;
        let mut removed = 0;
        for (i, backup) in backups.iter().enumerate() {
            if i >= keep_count {
                if fs::remove_dir_all(&backup.path).is_ok() {
                    removed += 1;
                }
            }
        }
        Ok(removed)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BackupInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub created: String,
}

pub fn copy_directory_recursive(src: &Path, dst: &Path) -> AppResult<()> {
    let mut skipped = Vec::new();
    copy_directory_recursive_inner(src, dst, &mut skipped)
}

/// 递归拷贝目录。容错策略：
/// ① 跳过符号链接/junction（pnpm 依赖链接指向 .pnpm 存储，不属于插件本体，跟随拷贝会误入依赖树甚至报拒绝访问）；
/// ② 单个文件拷贝失败（如被运行中进程独占锁定的日志/数据库）只记录并跳过，不使整个备份失败。
fn copy_directory_recursive_inner(src: &Path, dst: &Path, skipped: &mut Vec<String>) -> AppResult<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        // 用 symlink_metadata 识别链接本身（is_dir/is_file 会跟随链接）
        let meta = match fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(e) => {
                skipped.push(format!("{}: {}", path.display(), e));
                continue;
            }
        };
        if meta.file_type().is_symlink() {
            skipped.push(format!("{} (链接已跳过)", path.display()));
            continue;
        }

        if meta.is_dir() {
            if let Err(e) = copy_directory_recursive_inner(&path, &dest_path, skipped) {
                skipped.push(format!("{}: {}", path.display(), e));
            }
        } else {
            if let Some(parent) = dest_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            if let Err(e) = fs::copy(&path, &dest_path) {
                skipped.push(format!("{}: {}", path.display(), e));
            }
        }
    }
    Ok(())
}

fn strip_top_level_dir(path: &Path) -> PathBuf {
    let components: Vec<_> = path.components().collect();
    if components.len() > 1 {
        let mut result = PathBuf::new();
        for comp in &components[1..] {
            result.push(comp);
        }
        result
    } else {
        path.to_path_buf()
    }
}

/// 路径穿越防护（V2 §5 / F3）：拒绝含「..」（ParentDir）、根（RootDir）、
/// Windows 盘符前缀（Prefix）组件的条目路径；仅放行普通名称与当前目录（CurDir）。
/// zip 与 tgz 两条解压路径在 strip_top_level_dir 之后都必须经过本函数，
/// 被拒条目由调用方 log::warn 后跳过，不中断整个解压。
pub fn sanitize_rel_path(p: &Path) -> Option<PathBuf> {
    let mut cleaned = PathBuf::new();
    for comp in p.components() {
        match comp {
            std::path::Component::Normal(_) | std::path::Component::CurDir => {
                cleaned.push(comp.as_os_str());
            }
            std::path::Component::ParentDir
            | std::path::Component::RootDir
            | std::path::Component::Prefix(_) => return None,
        }
    }
    Some(cleaned)
}

fn get_dir_size(path: &Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                total += get_dir_size(&p);
            } else if let Ok(meta) = entry.metadata() {
                total += meta.len();
            }
        }
    }
    total
}

pub fn open_in_file_manager(path: &str) -> AppResult<()> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(AppError::DirectoryNotFound(path.to_string()));
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg("/select,")
            .arg(path)
            .spawn()
            .map_err(|e| AppError::Other(format!("Failed to open file manager: {}", e)))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-R")
            .arg(path)
            .spawn()
            .map_err(|e| AppError::Other(format!("Failed to open Finder: {}", e)))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| AppError::Other(format!("Failed to open file manager: {}", e)))?;
    }

    Ok(())
}

pub fn clean_temp_file(path: &str) {
    let _ = fs::remove_file(path);
}
/// 计算文件的 SHA256 校验和（返回小写 hex 字符串）
pub fn calculate_sha256(file_path: &str) -> AppResult<String> {
    use sha2::{Digest, Sha256};
    
    let mut file = fs::File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::write::GzEncoder;
    use std::io::Write;

    fn unique_temp_dir(tag: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!(
            "dsh_updater_test_{}_{}_{}",
            tag,
            std::process::id(),
            nanos
        ));
        fs::create_dir_all(&dir).expect("创建临时目录失败");
        dir
    }

    fn collect_files(root: &Path) -> Vec<PathBuf> {
        walkdir::WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .collect()
    }

    /// 手写 GNU 头字节构造恶意 tgz 条目（tar::Builder 会拒绝 `..` 路径，
    /// 而真实攻击者可控任意头字段）：目录项 pkg/../../evil/、文件项 pkg/../../file.txt、
    /// 绝对路径项 /abs/evil.txt（对应 V2 验收清单第 1 项的 fixture）
    fn append_raw_entry(buf: &mut Vec<u8>, name: &str, entry_type: tar::EntryType, data: &[u8]) {
        let mut header = tar::Header::new_gnu();
        header.set_entry_type(entry_type);
        header.set_size(data.len() as u64);
        header.set_mode(if entry_type.is_dir() { 0o755 } else { 0o644 });
        header.set_mtime(0);
        {
            let gnu = header.as_gnu_mut().expect("GNU 头");
            let bytes = name.as_bytes();
            let max = bytes.len().min(gnu.name.len());
            gnu.name[..max].copy_from_slice(&bytes[..max]);
        }
        header.set_cksum();
        buf.extend_from_slice(header.as_bytes());
        buf.extend_from_slice(data);
        let pad = (512 - data.len() % 512) % 512;
        buf.extend(std::iter::repeat(0u8).take(pad));
    }

    fn build_malicious_tgz(path: &Path) {
        let mut tar_buf = Vec::new();
        // ① 目录项：穿越到 target 之外（extract_tgz 目录分支原为裸 create_dir_all）
        append_raw_entry(&mut tar_buf, "pkg/../../evil/", tar::EntryType::Directory, b"");
        // ② 文件项：穿越到 target 之外
        append_raw_entry(&mut tar_buf, "pkg/../../file.txt", tar::EntryType::Regular, b"evil");
        // ③ 绝对路径项
        append_raw_entry(&mut tar_buf, "/abs/evil.txt", tar::EntryType::Regular, b"evil");
        // 结束块：两个 512 零块
        tar_buf.extend_from_slice(&[0u8; 1024]);

        let file = fs::File::create(path).expect("创建 tgz 失败");
        let mut enc = GzEncoder::new(file, flate2::Compression::default());
        enc.write_all(&tar_buf).expect("写入 tar 数据失败");
        enc.finish().expect("gzip 收尾失败");
    }

    #[test]
    fn malicious_tgz_cannot_escape_target() {
        let base = unique_temp_dir("malicious");
        let target = base.join("target");
        fs::create_dir_all(&target).expect("创建 target 失败");

        let tgz = base.join("evil.tgz");
        build_malicious_tgz(&tgz);

        // 解压前基线（target 内为空）
        let before = collect_files(&base);

        let fm = PluginFileManager::new(&target.to_string_lossy());
        // 被拒条目应跳过而非中断：整体解压成功返回
        fm.extract_update_package(&tgz.to_string_lossy(), &target.to_string_lossy())
            .expect("解压应成功（恶意条目仅跳过）");

        // 断言：target 目录之外无任何新增文件
        let after = collect_files(&base);
        let escaped: Vec<_> = after
            .iter()
            .filter(|p| !before.contains(p) && !p.starts_with(&target))
            .collect();
        assert!(escaped.is_empty(), "发现目录逃逸文件: {:?}", escaped);

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn legit_tgz_still_extracts() {
        let base = unique_temp_dir("legit");
        let target = base.join("target");
        fs::create_dir_all(&target).expect("创建 target 失败");

        // 正常 npm 包结构：顶层 package/ 前缀 + 目录项 + 文件项
        let tgz = base.join("good.tgz");
        let file = fs::File::create(&tgz).expect("创建 tgz 失败");
        let enc = GzEncoder::new(file, flate2::Compression::default());
        let mut builder = tar::Builder::new(enc);
        let mut h = tar::Header::new_gnu();
        h.set_entry_type(tar::EntryType::Directory);
        h.set_size(0);
        h.set_mode(0o755);
        h.set_cksum();
        builder
            .append_data(&mut h, "package/assets/", std::io::empty())
            .expect("写入目录项失败");
        let mut h2 = tar::Header::new_gnu();
        h2.set_entry_type(tar::EntryType::Regular);
        h2.set_size(2);
        h2.set_mode(0o644);
        h2.set_cksum();
        builder
            .append_data(&mut h2, "package/lib/index.js", b"ok".as_slice())
            .expect("写入文件项失败");
        let gz = builder.into_inner().expect("tar 收尾失败");
        gz.finish().expect("gzip 收尾失败");

        let fm = PluginFileManager::new(&target.to_string_lossy());
        fm.extract_update_package(&tgz.to_string_lossy(), &target.to_string_lossy())
            .expect("正常包解压应成功");

        assert!(target.join("assets").is_dir(), "目录项应解出 target/assets");
        assert!(
            target.join("lib").join("index.js").is_file(),
            "文件项应解出 target/lib/index.js"
        );

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn sanitize_rel_path_rejects_dangerous_components() {
        assert!(sanitize_rel_path(Path::new("..")).is_none());
        assert!(sanitize_rel_path(Path::new("a/../../b")).is_none());
        assert!(sanitize_rel_path(Path::new("a/../b")).is_none());
        assert!(sanitize_rel_path(Path::new("/abs")).is_none());
        assert!(sanitize_rel_path(Path::new("C:\\evil\\x")).is_none());
        assert!(sanitize_rel_path(Path::new("normal/ok.txt")).is_some());
        assert!(sanitize_rel_path(Path::new("./rel/file")).is_some());
    }
}

