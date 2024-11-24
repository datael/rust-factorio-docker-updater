use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct BranchAndChannelPair {
    branch: String,
    channel: String,
}

impl Display for BranchAndChannelPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}-{}", self.branch, self.channel)
    }
}

impl BranchAndChannelPair {
    pub fn new(branch: impl Into<String>, channel: impl Into<String>) -> Self {
        Self {
            branch: branch.into(),
            channel: channel.into(),
        }
    }
}
