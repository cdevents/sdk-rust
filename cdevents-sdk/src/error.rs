use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    UriParseError( #[from] fluent_uri::ParseError),
    #[error("unknown error")]
    Unknown,
}
