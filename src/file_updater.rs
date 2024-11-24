use std::path::Path;

use anyhow::Result;

pub mod docker_compose_file;

pub trait FileUpdater {
    type Request;

    async fn update_file(path: impl AsRef<Path>, request: &Self::Request) -> Result<()>;
}
