#![doc = include_str!("../../README.md")]
//#![warn(missing_docs)]
// TODO remove unwrap(), expect(...)
// TODO reduce clone()
mod cdevent;
mod context;
#[cfg(feature = "cloudevents")]
pub mod cloudevents;
mod error;
mod generated;
pub(crate) mod serde;
mod subject;
mod uri;
mod uri_reference;

pub use cdevent::*;
pub use context::*;
pub use error::*;
pub use generated::*;
pub use subject::*;
pub use uri::*;
pub use uri_reference::*;
