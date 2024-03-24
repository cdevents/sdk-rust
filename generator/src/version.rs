use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct Version {
    pub(crate) major: u8,
    pub(crate) minor: u8,
    pub(crate) patch: u8,
    pub(crate) modifier: String,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut r = self.major.cmp(&other.major);
        if r.is_eq() {
            r = self.minor.cmp(&other.minor);
        }
        if r.is_eq() {
            r = self.patch.cmp(&other.patch);
        }
        //TODO compare modifier with semantic ("snapshot", "draft", "alpha", "beta", "rc", "ga", "")
        r
    }
}
