[![releaser](https://github.com/353fc443/github-bin-downloader/actions/workflows/release.yaml/badge.svg)](https://github.com/353fc443/github-bin-downloader/actions/workflows/release.yaml)
[![GitHub license](https://img.shields.io/github/license/353fc443/github-bin-downloader)](https://github.com/353fc443/github-bin-downloader/blob/main/LICENSE)
<a href="https://crates.io/crates/github-bin-downloader"><img src="https://img.shields.io/crates/v/github-bin-downloader.svg" alt="Version info"></a><br>

# github-bin-downloader

Download binary for your OS from Github. 
## Installation 

Install github-bin-downloader using cargo

```shell 
cargo install github-bin-downloader
```

## Demo

![Demo](static/demo.gif)

## Usage

```shell
github-bin-downloader 0.1.0

USAGE:
    github-bin-downloader [FLAGS] --url <url>

FLAGS:
    -h, --help       Prints help information
        --latest     Check for the latest release including prerelease
        --list       View all files as a list
    -V, --version    Prints version information

OPTIONS:
    -u, --url <url>    Github repository URL
```

## License
[GNU GPL](https://choosealicense.com/licenses/gpl-3.0/)
