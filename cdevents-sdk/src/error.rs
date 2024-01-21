use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Empty data in cloudevent")]
    DataNotFoundInCloudEvent,
    #[error(transparent)]
    UriParseError( #[from] fluent_uri::ParseError),
    #[error(transparent)]
    SerdeJsonError( #[from] serde_json::Error),
    #[error("unknown error")]
    Unknown,
}
