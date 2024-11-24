use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Version(String);

impl<S> From<S> for Version
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        Self(value.as_ref().to_string())
    }
}

impl Deref for Version {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
