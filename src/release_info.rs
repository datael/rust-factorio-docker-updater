use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::join;

pub mod available_channels;
pub mod branch_and_channel;
pub mod http;
pub mod latest_releases;
pub mod sha256;
pub mod version;

pub use latest_releases::LatestReleases;
pub use sha256::{Sha256Sum, Sha256Sums};
pub use version::Version;

pub const LATEST_RELEASES_URL_DEFAULT: &str = "https://factorio.com/api/latest-releases";
pub const SHA256SUMS_URL_DEFAULT: &str = "https://factorio.com/download/sha256sums/";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReleaseInfo<'a> {
    pub latest_releases_url: &'a str,
    pub sha256sums_url: &'a str,
}

impl<'a> Default for ReleaseInfo<'a> {
    fn default() -> Self {
        Self {
            latest_releases_url: LATEST_RELEASES_URL_DEFAULT,
            sha256sums_url: SHA256SUMS_URL_DEFAULT,
        }
    }
}

impl ReleaseInfo<'_> {
    pub async fn get_latest_release_info(&self, pair: String) -> Result<(Version, Sha256Sum)> {
        let (latest_releases, sha256sums) = join!(
            LatestReleases::get_from(self.latest_releases_url),
            Sha256Sums::get_from(self.sha256sums_url)
        );

        let (latest_releases, sha256sums) = (latest_releases?, sha256sums?);

        let version = latest_releases.get_version(pair)?;
        let filename = format!("factorio-headless_linux_{version}.tar.xz");

        let sha256sum = sha256sums.get_sha256sum_for_file(filename)?;

        Ok((version.clone(), sha256sum.clone()))
    }
}
