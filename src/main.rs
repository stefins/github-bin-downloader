use github_bin_downloader::{cli, ghapi, utils};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = cli::run_cli();
    if opt.latest {
        let mut repo = ghapi::RepoInfo::from_url(&opt.url).await?;
        repo.get_latest_release().await?;
        if opt.list {
            cli::display_all_options(&repo.releases)
                .await?
                .download_release()
                .await?;
            return Ok(());
        }
        match utils::compare_two_vector(
            &repo.search_releases_for_os().await?,
            &repo.search_releases_for_arch().await?,
        ) {
            Some(releases) => {
                cli::display_all_options(&releases).await?
                .download_release().await?;
                return Ok(());
            },
            None => println!("Cannot find a release for you OS and Arch\n Use --list flag to list all available options"),
        }
    } else {
        let mut repo = ghapi::RepoInfo::from_url(&opt.url).await?;
        repo.get_latest_stable_release().await?;
        if opt.list {
            cli::display_all_options(&repo.releases)
                .await?
                .download_release()
                .await?;
            return Ok(());
        }
        match utils::compare_two_vector(
            &repo.search_releases_for_os().await?,
            &repo.search_releases_for_arch().await?,
        ) {
            Some(releases) => {
                cli::display_all_options(&releases).await?
                .download_release().await?;
                return Ok(());
            },
            None => println!("Cannot find a release for you OS and Arch\n Use --list flag to list all available options"),
        }
    }
    Ok(())
}
