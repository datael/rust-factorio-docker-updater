use std::{collections::HashMap, ops::Deref};

use serde::{Deserialize, Serialize};

use super::version::Version;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AvailableChannels(HashMap<String, Version>);

impl Deref for AvailableChannels {
    type Target = HashMap<String, Version>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
