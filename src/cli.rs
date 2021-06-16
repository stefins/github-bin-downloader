use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "github-bin-downloader")]
pub struct Opt {
    #[structopt(short, long, required = true, help = "Github repository URL")]
    pub url: String,
    #[structopt(long, help = "Check for the latest release including prerelease")]
    pub latest: bool,
    #[structopt(long)]
    pub list: bool,
}

pub fn run_cli() -> Opt {
    Opt::from_args()
}
