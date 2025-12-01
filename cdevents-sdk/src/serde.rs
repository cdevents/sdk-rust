// currently there is no generic way to handle Option<T>
// 2 alertnatives:
// - write custom serializer
// - try to use the crate serde_with (serde_as)
//
// Creating, aliasing module with custom (de)serializer,
// also simplify the generation and introduce a small layer of abstraction/documentation.
//
// see [Using de/serialize\_with inside of an Option, Map, Vec · Issue #723 · serde-rs/serde](https://github.com/serde-rs/serde/issues/723)

pub(crate) mod datetime {
    use serde::{Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<time::OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        time::serde::rfc3339::deserialize(deserializer)
    }

    pub fn serialize<S>(input: &time::OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        time::serde::rfc3339::serialize(input, serializer)
    }
}

// pub(crate) mod fluent_uri {
//     use serde::{de::Error, Deserialize, Deserializer, Serializer};

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<fluent_uri::UriRef<String>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let txt = String::deserialize(deserializer)?;
//         fluent_uri::UriRef::parse(txt).map_err(|e| match e {
//                 fluent_uri::
//             }
//             D::Error::custom)
//     }

//     pub fn serialize<S>(input: &fluent_uri::UriRef<String>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.collect_str(input.as_str())
//     }
// }
