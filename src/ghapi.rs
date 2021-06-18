use crate::sysinfo;
use crate::utils;

use crate::GBDResult;
use reqwest::{StatusCode, Url};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct RepoInfo {
    user_name: String,
    repo_name: String,
    url: String,
    releases_api_url: String,
    pub releases: Vec<Release>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Release {
    pub name: String,
    pub url: String,
}

impl Release {
    // Download the release
    pub async fn download_release(&self) -> GBDResult<()> {
        let url = Url::parse(self.url.as_str())?;
        println!("Downloading {} from {}", self.name, url);
        utils::download_file_from_url(url, &self.name).await?;
        Ok(())
    }
}

impl ToString for Release {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Error, Debug)]
pub enum GithubError {
    #[error("Repo Not Found")]
    NotFound(StatusCode),
}

impl RepoInfo {
    pub async fn from_url(url: &str) -> GBDResult<Self> {
        let mut url = url.to_string();
        if !url.contains("https://") && !url.contains("http://") {
            url = format!("https://{}", url);
        }
        if !url.contains("github") {
            return Err(Box::new(GithubError::NotFound(StatusCode::NOT_IMPLEMENTED)));
        }
        let resp = reqwest::get(&url).await?;
        if resp.status() == StatusCode::OK {
            let path = resp.url().path();
            let repoinfo_vec: Vec<&str> = path.split('/').collect();
            let releases_api_url = format!(
                "https://api.github.com/repos/{}/{}/releases",
                repoinfo_vec[1].to_string(),
                repoinfo_vec[2].to_string()
            );
            Ok(RepoInfo {
                user_name: repoinfo_vec[1].to_string(),
                repo_name: repoinfo_vec[2].to_string(),
                url,
                releases_api_url,
                ..Default::default()
            })
        } else {
            Err(Box::new(GithubError::NotFound(resp.status())))
        }
    }

    // Fetch the latest release from Github including Pre-release
    pub async fn get_latest_release(&mut self) -> GBDResult<()> {
        let client = reqwest::Client::builder().user_agent("curl").build()?;
        let resp = client
            .get(&self.releases_api_url)
            .send()
            .await?
            .text()
            .await?;
        let repo: Value = serde_json::from_str(&resp)?;
        let length = repo[0]["assets"].as_array().unwrap().len();
        let mut releases: Vec<Release> = Vec::new();
        for i in 0..length {
            releases.push(Release {
                name: utils::sanitize_str_to_string(&repo[0]["assets"][i]["name"]),
                url: utils::sanitize_str_to_string(&repo[0]["assets"][i]["browser_download_url"]),
            });
        }
        self.releases = releases;
        Ok(())
    }

    // Get all the latest stable releases from Github releases
    pub async fn get_latest_stable_release(&mut self) -> GBDResult<()> {
        let client = reqwest::Client::builder().user_agent("curl").build()?;
        let resp = client
            .get(&self.releases_api_url)
            .send()
            .await?
            .text()
            .await?;
        let repo: Value = serde_json::from_str(&resp)?;
        let length = repo.as_array().unwrap().len();
        let mut releases: Vec<Release> = Vec::new();
        for i in 0..length {
            if !repo[i]["prerelease"]
                .as_bool()
                .expect("Cannot convert to bool")
            {
                let length = repo[i]["assets"].as_array().unwrap().len();
                for j in 0..length {
                    releases.push(Release {
                        name: utils::sanitize_str_to_string(&repo[i]["assets"][j]["name"]),
                        url: utils::sanitize_str_to_string(
                            &repo[i]["assets"][j]["browser_download_url"],
                        ),
                    });
                }
                self.releases = releases;
                return Ok(());
            }
        }
        Ok(())
    }

    // Search the releases for the host OS
    pub async fn search_releases_for_os(&self) -> GBDResult<Vec<Release>> {
        let sys_info = sysinfo::SystemInfo::new();
        let mut releases: Vec<Release> = Vec::new();
        match sys_info.platform_os() {
            sysinfo::PlatformOS::Darwin => {
                sysinfo::APPLE.iter().for_each(|mac| {
                    self.releases.iter().for_each(|release| {
                        if release.name.to_lowercase().contains(mac) {
                            releases.push(release.clone());
                        }
                    });
                });
            }
            sysinfo::PlatformOS::Linux => {
                sysinfo::LINUX.iter().for_each(|linux| {
                    self.releases.iter().for_each(|release| {
                        if release.name.to_lowercase().contains(linux) {
                            releases.push(release.clone());
                        }
                    });
                });
            }
            _ => {}
        }
        Ok(releases)
    }

    // Search the releases for the host Arch
    pub async fn search_releases_for_arch(&self) -> GBDResult<Vec<Release>> {
        let sys_info = sysinfo::SystemInfo::new();
        let mut releases: Vec<Release> = Vec::new();
        match sys_info.platform_arch() {
            sysinfo::PlatformArch::X8664 => {
                sysinfo::AMD64.iter().for_each(|arch| {
                    self.releases.iter().for_each(|release| {
                        if release.name.contains(arch) {
                            releases.push(release.clone());
                        }
                    });
                });
            }
            sysinfo::PlatformArch::Arm64 => {
                sysinfo::ARM64.iter().for_each(|arch| {
                    self.releases.iter().for_each(|release| {
                        if release.name.contains(arch) {
                            releases.push(release.clone());
                        }
                    });
                });
            }
            _ => {}
        }
        Ok(releases)
    }
}
