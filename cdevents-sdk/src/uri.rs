// wrapper for fluent_uri::Uri to allow for restristed set of operations
// and to complete currently missing features.
//TODO impl the check difference for URI and Uri

use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::UriReference;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Uri(#[serde(with = "crate::serde::fluent_uri")] pub(crate) fluent_uri::Uri<String>);

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
