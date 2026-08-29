use crate::error::{AppError, AppResult, PluginManifest};
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
