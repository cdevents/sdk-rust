// wrapper for fluent_uri::Uri to allow for restristed set of operations
// and to complete currently missing features.
// Why fluent_uri?
// - support uri & uri-reference, preserve the original string, but young, doesn't impl PartialEq,...
// - http::Uri, more mature, but doesn't support uri-reference, and normalize url when generate string
//TODO impl the check difference for Uri and Uri-reference

use std::str::FromStr;

use serde::{Serialize, Deserialize};
#[cfg(feature = "testkit")] use proptest_derive::Arbitrary;
use crate::UriReference;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
pub struct Uri(
    #[cfg_attr(feature = "testkit", proptest(value = "fluent_uri::Uri::parse_from(\"https://example.com/\".to_owned()).unwrap()"))] //TODO generate random value
    #[serde(with = "crate::serde::fluent_uri")]
    pub(crate) fluent_uri::Uri<String>
);

impl PartialEq for Uri {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for Uri {}

impl FromStr for Uri {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //TODO check it's not a reference URI
        fluent_uri::Uri::parse_from(s.to_owned()).map_err(|(_,e)| e.into()).map(Uri)
    }
}

impl TryFrom<UriReference> for Uri {
    type Error = crate::Error;

    fn try_from(s: UriReference) -> Result<Self, Self::Error> {
        Uri::from_str(s.as_str())
    }
}

impl TryFrom<&str> for Uri {
    type Error = crate::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fluent_uri::Uri::parse_from(s.to_owned()).map_err(|(_,e)| e.into()).map(Uri)
    }
}

impl TryFrom<String> for Uri {
    type Error = crate::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        fluent_uri::Uri::parse_from(s).map_err(|(_,e)| e.into()).map(Uri)
    }
}

impl ToString for Uri {
    fn to_string(&self) -> String {
        self.0.as_str().to_owned()//into_string()
    }
}

impl Uri {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// impl From<Uri> for fluent_uri::Uri<String> {
//     fn from(uri: Uri) -> Self {
//         uri.0
//     }
// }

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        #[cfg(feature = "testkit")]
        fn arbitraries_are_json_valid(s in any::<Uri>()) {
            let json_str = serde_json::to_string(&s).unwrap();
            let actual = serde_json::from_str::<Uri>(&json_str).unwrap();
            assert_eq!(s, actual);
        }
    }
}
