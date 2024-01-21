use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::Uri;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UriReference(#[serde(with = "crate::serde::fluent_uri")] pub(crate) fluent_uri::Uri<String>);

impl PartialEq for UriReference {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for UriReference {}

impl FromStr for UriReference {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fluent_uri::Uri::parse_from(s.to_owned()).map_err(|(_,e)| e.into()).map(UriReference)
    }
}

impl TryFrom<Uri> for UriReference {
    type Error = crate::Error;

    fn try_from(s: Uri) -> Result<Self, Self::Error> {
        Ok(UriReference(s.0))
    }
}

impl TryFrom<&str> for UriReference {
    type Error = crate::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fluent_uri::Uri::parse_from(s.to_owned()).map_err(|(_,e)| e.into()).map(UriReference)
    }
}

impl TryFrom<String> for UriReference {
    type Error = crate::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        fluent_uri::Uri::parse_from(s).map_err(|(_,e)| e.into()).map(UriReference)
    }
}

impl ToString for UriReference {
    fn to_string(&self) -> String {
        self.0.as_str().to_owned()//into_string()
    }
}

impl UriReference {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// impl From<UriReference> for fluent_uri::Uri<String> {
//     fn from(uri: UriReference) -> Self {
//         uri.0
//     }
// }
