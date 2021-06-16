use github_bin_downloader::{cli, ghapi};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = cli::run_cli();
    if opt.latest {
        let mut repo = ghapi::RepoInfo::from_url(&opt.url).await?;
        repo.get_latest_release().await?;
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
    } else {
        let mut repo = ghapi::RepoInfo::from_url(&opt.url).await?;
        repo.get_latest_stable_release().await?;
        println!("{:?}", repo.search_releases_for_os().await?);
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
    }
    println!("Cannot find a release for your OS!");
    Ok(())
}
