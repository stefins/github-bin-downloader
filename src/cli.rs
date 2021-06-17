use dialoguer::Select;
use structopt::StructOpt;

use crate::ghapi::Release;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "github-bin-downloader",
    about = "Download binary for your OS from Github releases."
)]
pub struct Opt {
    #[structopt(short, long, required = true, help = "Github repository URL")]
    pub url: String,
    #[structopt(long, help = "Check for the latest release including prerelease")]
    pub latest: bool,
    #[structopt(long, help = "View all files as a list")]
    pub list: bool,
}

pub fn run_cli() -> Opt {
    Opt::from_args()
}

pub async fn display_all_options(
    releases: &[Release],
) -> Result<Release, Box<dyn std::error::Error>> {
    if releases.is_empty() {
        println!("No releases available!");
        std::process::exit(1)
    }
    println!("Select the release you want to download!");
    let selection = Select::new().items(&releases).default(0).interact()?;
    Ok(releases[selection].clone())
}
