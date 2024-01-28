use std::str::FromStr;

use serde::{Serialize, Deserialize};
use crate::Uri;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UriReference(
    #[serde(with = "crate::serde::fluent_uri")]
    pub(crate) fluent_uri::Uri<String>
);

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

#[cfg(feature = "testkit")]
impl<> proptest::arbitrary::Arbitrary for UriReference {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        (prop_oneof![
            "\\/[a-z_\\-\\/]+".prop_map(|s| UriReference::from_str(&s).unwrap()),
            Just("https://example.com/").prop_map(|s| UriReference::from_str(&s).unwrap()),
        ]).boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        #[cfg(feature = "testkit")]
        fn arbitraries_are_json_valid(s in any::<UriReference>()) {
            let json_str = serde_json::to_string(&s).unwrap();
            let actual = serde_json::from_str::<UriReference>(&json_str).unwrap();
            assert_eq!(s, actual);
        }
    }
}
