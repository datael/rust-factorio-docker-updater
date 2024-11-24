use std::{collections::HashMap, ops::Deref};

use anyhow::Result;
use itertools::*;
use serde::{Deserialize, Serialize};

use super::{
    available_channels::AvailableChannels, branch_and_channel::BranchAndChannelPair, http::Http,
    version::Version,
};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct LatestReleases(HashMap<String, AvailableChannels>);

impl Deref for LatestReleases {
    type Target = HashMap<String, AvailableChannels>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LatestReleases {
    pub async fn get_from(url: &str) -> Result<Self> {
        Http::get_from::<Self>(url).await
    }

    pub fn get_version(&self, branch_and_channel_pair: impl AsRef<str>) -> Result<&Version> {
        let (branch, channel) = branch_and_channel_pair
            .as_ref()
            .split_once("-")
            .ok_or_else(|| {
                anyhow::format_err!(
                    "Failed to parse pair '{0}'. Available: {1}",
                    branch_and_channel_pair.as_ref(),
                    self.get_available_channel_pairs_string()
                )
            })?;

        let version = self
            .get(branch)
            .ok_or_else(|| {
                anyhow::format_err!(
                    "Branch '{0}' from pair '{1}' not found. Available: {2}",
                    branch,
                    branch_and_channel_pair.as_ref(),
                    self.get_available_channel_pairs_string()
                )
            })?
            .get(channel)
            .ok_or_else(|| {
                anyhow::format_err!(
                    "Channel '{0}' from pair '{1}' not found. Available: {2}",
                    channel,
                    branch_and_channel_pair.as_ref(),
                    self.get_available_channel_pairs_string()
                )
            })?;

        Ok(version)
    }

    pub fn get_available_channel_pairs(&self) -> Vec<BranchAndChannelPair> {
        self.iter()
            .flat_map(|(branch, channels)| {
                channels
                    .keys()
                    .map(|channel| BranchAndChannelPair::new(branch.clone(), channel))
            })
            .collect()
    }

    fn get_available_channel_pairs_string(&self) -> String {
        self.get_available_channel_pairs()
            .iter()
            .map(|pair| format!("{pair}"))
            .join(", ")
    }
}
