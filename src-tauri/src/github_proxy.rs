use crate::error::{AppError, AppResult, GitHubRelease, PluginInfo};
use reqwest::Client;
use semver::Version;
use std::time::Duration;

pub struct GitHubProxyClient {
    base_url: String,
    client: Client,
    api_token: Option<String>,
}

impl GitHubProxyClient {
    /// 暴露内部 HTTP 客户端给目录拉取等模块复用
    pub fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn new(base_url: &str, api_token: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        GitHubProxyClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
            api_token,
        }
    }

    /// 获取最新 release：先尝试代理，失败自动降级直连 GitHub API
    /// 返回 (release, via_proxy)
    pub async fn fetch_latest_release_ex(&self, repo: &str) -> AppResult<(GitHubRelease, bool)> {
        if repo.is_empty() {
            return Err(AppError::Other("GitHub repository address is empty".to_string()));
        }

        // 1. 尝试代理（若配置了）
        if !self.base_url.is_empty() {
            match self.fetch_latest_release_via_proxy(repo).await {
                Ok(r) => return Ok((r, true)),
                Err(proxy_err) => {
                    eprintln!(
                        "[github_proxy] proxy {} failed: {}, falling back to direct GitHub API",
                        self.base_url, proxy_err
                    );
                }
            }
        }

        // 2. 直连 GitHub API
        let release = self.fetch_latest_release_direct(repo).await?;
        Ok((release, false))
    }

    pub async fn fetch_latest_release(&self, repo: &str) -> AppResult<GitHubRelease> {
        self.fetch_latest_release_ex(repo).await.map(|(r, _)| r)
    }

    async fn fetch_latest_release_via_proxy(&self, repo: &str) -> AppResult<GitHubRelease> {
        let url = format!(
            "{}/api/github/latest?repo={}",
            self.base_url,
            urlencoding::encode(repo)
        );

        let mut request = self.client.get(&url);
        if let Some(token) = &self.api_token {
            request = request.header("X-Proxy-Token", token);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::Other(format!(
                "Proxy request failed [{}]: {}",
                status, text
            )));
        }

        let release: GitHubRelease = response.json().await?;
        Ok(release)
    }

    /// 直连 GitHub API（无需代理）
    async fn fetch_latest_release_direct(&self, repo: &str) -> AppResult<GitHubRelease> {
        let url = format!("https://api.github.com/repos/{}/releases/latest", repo);

        let mut request = self
            .client
            .get(&url)
            .header("User-Agent", "dsh-plugin-updater")
            .header("Accept", "application/vnd.github+json");

        if let Some(token) = &self.api_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::Other(format!(
                "GitHub API request failed [{}]: {}",
                status, text
            )));
        }

        let release: GitHubRelease = response.json().await?;
        Ok(release)
    }

    pub async fn fetch_releases(&self, repo: &str, per_page: u8) -> AppResult<Vec<GitHubRelease>> {
        let url = if self.base_url.is_empty() {
            // 本地直连：GitHub API
            format!(
                "https://api.github.com/repos/{}/releases?per_page={}",
                repo,
                per_page
            )
        } else {
            format!(
                "{}/api/github/releases?repo={}&per_page={}",
                self.base_url,
                urlencoding::encode(repo),
                per_page
            )
        };

        let mut request = self.client.get(&url);
        if let Some(token) = &self.api_token {
            request = request.header("X-Proxy-Token", token);
        }

        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(AppError::Other(format!(
                "Proxy request failed: {}",
                response.status()
            )));
        }

        let releases: Vec<GitHubRelease> = response.json().await?;
        Ok(releases)
    }

    pub fn get_download_url(&self, repo: &str, tag: &str, asset_name: &str) -> String {
        if self.base_url.is_empty() {
            // 本地直连：GitHub 标准下载地址
            format!(
                "https://github.com/{}/releases/download/{}/{}",
                repo, tag, asset_name
            )
        } else {
            format!(
                "{}/api/github/download?repo={}&tag={}&asset={}",
                self.base_url,
                urlencoding::encode(repo),
                urlencoding::encode(tag),
                urlencoding::encode(asset_name)
            )
        }
    }

    pub async fn check_plugin_update(
        &self,
        plugin: &mut PluginInfo,
    ) -> Result<(), String> {
        let repo = &plugin.manifest.github_repo;
        if repo.is_empty() {
            plugin.check_error = Some("GitHub repository not configured".to_string());
            return Err("GitHub repository not configured".to_string());
        }

        match self.fetch_latest_release_ex(repo).await {
            Ok((release, via_proxy)) => {
                let latest_version = release.tag_name.trim_start_matches('v').to_string();
                plugin.latest_version = Some(latest_version.clone());
                plugin.release_url = Some(release.html_url.clone());
                plugin.release_notes = Some(release.body.clone());

                // Find suitable download asset (.zip or .tar.gz)
                if let Some(asset) = release.assets.iter().find(|a| {
                    a.name.ends_with(".zip") || a.name.ends_with(".tar.gz")
                }) {
                    if via_proxy {
                        plugin.download_url = Some(self.get_download_url(
                            repo,
                            &release.tag_name,
                            &asset.name,
                        ));
                    } else {
                        // 直连时使用 GitHub 真实下载地址
                        plugin.download_url = Some(asset.browser_download_url.clone());
                    }
                }

                // Version comparison
                plugin.update_available = compare_versions(
                    &plugin.manifest.current_version,
                    &latest_version,
                );

                Ok(())
            }
            Err(e) => {
                let err_msg = e.to_string();
                plugin.check_error = Some(err_msg.clone());
                Err(err_msg)
            }
        }
    }

    pub async fn check_all_updates(
        &self,
        plugins: &mut [PluginInfo],
    ) -> Vec<(String, Result<bool, String>)> {
        let mut results = Vec::new();
        for plugin in plugins.iter_mut() {
            let id = plugin.manifest.id.clone();
            let result = self.check_plugin_update(plugin).await;
            results.push((id, result.map(|_| plugin.update_available)));
        }
        results
    }

    pub async fn download_file_with_progress<F>(
        &self,
        url: &str,
        dest_path: &str,
        mut progress_callback: F,
    ) -> AppResult<()>
    where
        F: FnMut(u64, u64),
    {
        use futures_util::StreamExt;
        use std::fs::File;
        use std::io::Write;

        let mut request = self.client.get(url);
        if let Some(token) = &self.api_token {
            request = request.header("X-Proxy-Token", token);
        }

        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(AppError::Other(format!(
                "Download failed: HTTP {}",
                response.status()
            )));
        }

        let total_size = response.content_length().unwrap_or(0);
        let mut file = File::create(dest_path)?;
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;
            progress_callback(downloaded, total_size);
        }

        file.flush()?;
        Ok(())
    }
}

pub fn compare_versions(current: &str, latest: &str) -> bool {
    let parse_version = |v: &str| -> Option<Version> {
        let cleaned = v.trim_start_matches('v').trim();
        if cleaned.is_empty() {
            return None;
        }
        Version::parse(cleaned).ok()
    };

    match (parse_version(current), parse_version(latest)) {
        (Some(current_ver), Some(latest_ver)) => latest_ver > current_ver,
        // If either version is invalid, fall back to string comparison
        _ => latest.trim_start_matches('v').to_string() > current.trim_start_matches('v').to_string(),
    }
}

pub fn normalize_version(version: &str) -> String {
    let v = version.trim_start_matches('v').trim();
    if v.is_empty() {
        "0.0.0".to_string()
    } else {
        v.to_string()
    }
}
