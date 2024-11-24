use anyhow::Result;
use clap::Parser;

use release_info::*;

mod release_info;

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let release_info = ReleaseInfo {
        latest_releases_url: &args.latest_releases_url,
        sha256sums_url: &args.sha256sums_url,
    };

    let (version, sha256sum) = release_info.get_latest_release_info(args.pair).await?;

    println!("Version: {version}");
    println!("Sha256:  {sha256sum}");

    Ok(())
}
