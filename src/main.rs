use std::env;

use github_bin_downloader::ghapi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut repo = ghapi::RepoInfo::from_url(&args[1]).await?;
    repo.get_latest_stable_release().await?;
    for release in repo.search_releases_for_os().await? {
        for release_arch in repo.search_releases_for_arch().await? {
            if release_arch == release {
                release.download_release().await?;
                return Ok(());
            }
        }
    }
    if let Some(release) = repo.search_releases_for_os().await?.get(0) {
        release.download_release().await?;
    }
    Ok(())
}
