use anyhow::{Context, Result};
use bytes::Bytes;
use serde::de::DeserializeOwned;

pub struct Http;

impl Http {
    pub async fn get_bytes_from(url: impl AsRef<str>) -> Result<Bytes> {
        reqwest::get(url.as_ref())
            .await
            .with_context(|| format!("Failed to get {0}", url.as_ref()))?
            .bytes()
            .await
            .with_context(|| {
                format!(
                    "Failed to extract bytes from response received from {0}",
                    url.as_ref()
                )
            })
    }

    pub async fn get_from<'a, T>(url: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let bytes = Self::get_bytes_from(url.as_ref()).await?;
        let deserialized = serde_json::from_slice(&bytes)?;

        Ok(deserialized)
    }
}
