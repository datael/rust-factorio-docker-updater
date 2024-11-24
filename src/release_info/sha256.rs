use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncBufReadExt;

use super::http::Http;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Sha256Sums(HashMap<Filename, Sha256Sum>);

impl Sha256Sums {
    pub async fn get_from(url: &str) -> Result<Self> {
        let bytes = Http::get_bytes_from(url).await?;

        let mut sums = Sha256Sums::default();

        let mut sha256sums_raw = bytes.lines();
        while let Some(line) = sha256sums_raw.next_line().await? {
            if let Some((sha256sum, filename)) = line.split_once("  ") {
                sums.0
                    .insert(Filename::from(filename), Sha256Sum::from(sha256sum));
            }
        }

        Ok(sums)
    }

    pub fn get_sha256sum_for_file(&self, filename: impl AsRef<str>) -> Result<&Sha256Sum> {
        let sha256sum = self
            .0
            .get(&filename.as_ref().into())
            .ok_or_else(|| anyhow::format_err!("File {0}", filename.as_ref()))?;

        Ok(sha256sum)
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Sha256Sum(String);

impl<S> From<S> for Sha256Sum
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        Self(value.as_ref().to_string())
    }
}

impl Deref for Sha256Sum {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Sha256Sum {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Filename(String);

impl<S> From<S> for Filename
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        Self(value.as_ref().to_string())
    }
}

impl Deref for Filename {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Filename {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
