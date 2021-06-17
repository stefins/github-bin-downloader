use std::{cmp::min, fs::File, io::Write};

use crate::sysinfo;

use indicatif::{ProgressBar, ProgressStyle};
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
    pub async fn download_release(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse(self.url.as_str())?;
        println!("Downloading {} from {}", self.name, url);
        let mut resp = reqwest::get(url).await?;
        resp.content_length();
        let mut f = File::create(&self.name)?;
        let mut downloaded = 0;
        let total_size = resp.content_length().unwrap();
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-"));
        while let Some(chunk) = resp.chunk().await? {
            let new = min(downloaded + chunk.len() as u64, total_size);
            downloaded = new;
            pb.set_position(new);
            f.write_all(&chunk[..])?;
        }
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
    pub async fn from_url(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
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
            return Err(Box::new(GithubError::NotFound(resp.status())));
        }
    }

    pub async fn get_latest_release(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
                name: repo[0]["assets"][i]["name"].to_string().replace('"', ""),
                url: repo[0]["assets"][i]["browser_download_url"]
                    .to_string()
                    .replace('"', ""),
            });
        }
        self.releases = releases;
        Ok(())
    }

    pub async fn get_latest_stable_release(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
                        name: repo[i]["assets"][j]["name"].to_string().replace('"', ""),
                        url: repo[i]["assets"][j]["browser_download_url"]
                            .to_string()
                            .replace('"', ""),
                    });
                }
                self.releases = releases;
                return Ok(());
            }
        }
        Ok(())
    }

    pub async fn search_releases_for_os(&self) -> Result<Vec<Release>, Box<dyn std::error::Error>> {
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

    pub async fn search_releases_for_arch(
        &self,
    ) -> Result<Vec<Release>, Box<dyn std::error::Error>> {
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
