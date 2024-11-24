use anyhow::Result;
use clap::Parser;

mod file_updater;
mod release_info;

use docker_compose_file::*;
use file_updater::*;
use release_info::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        long,
        short = 'p',
        help = "Branch and Channel Pair, e.g. experimental-expansion"
    )]
    pair: String,

    #[arg(
        long,
        help = "Latest Releases URL",
        default_value= LATEST_RELEASES_URL_DEFAULT,
    )]
    latest_releases_url: String,

    #[arg(
        long,
        help = "SHA256 Sums URL",
        default_value=SHA256SUMS_URL_DEFAULT,
    )]
    sha256sums_url: String,

    #[arg(long, help = "Path to file to update")]
    path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let release_info = ReleaseInfo {
        latest_releases_url: &args.latest_releases_url,
        sha256sums_url: &args.sha256sums_url,
    };

    let (version, sha256sum) = release_info.get_latest_release_info(args.pair).await?;

    if let Some(target_path) = args.path {
        let request = UpdateRequest {
            target_version: &version,
            target_sha256sum: &sha256sum,
        };

        Updater::update_file(target_path, &request).await?;
    } else {
        println!("{version}  {sha256sum}");
    }

    Ok(())
}
