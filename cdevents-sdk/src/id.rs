use std::{fmt::{Display, Formatter}, str::FromStr};

use serde::{Deserialize, Serialize};

pub type Id = NonEmptyString;
pub type Name = NonEmptyString;

/// A non-empty string.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn as_str(&self) -> &str {
        &self.0.as_str()
    }
}

impl Default for NonEmptyString {
    fn default() -> Self {
        NonEmptyString("00000000-0000-0000-0000-000000000000".to_owned())
    }
}

impl From<NonEmptyString> for String {
    fn from(id: NonEmptyString) -> Self {
        id.0
    }
}
impl TryFrom<String> for NonEmptyString {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(crate::Error::EmptyString("id"))
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for NonEmptyString {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(crate::Error::EmptyString("id"))
        }
        Ok(Self(value.to_owned()))
    }
}

impl FromStr for NonEmptyString {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl Display for NonEmptyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&NonEmptyString> for String {
    fn from(id: &NonEmptyString) -> Self {
        id.0.clone()
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "testkit")]
impl<> proptest::arbitrary::Arbitrary for NonEmptyString {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        "\\PC+".prop_map(|id| 
            id.try_into().expect("generate valid id")
        ).boxed()
    }
}