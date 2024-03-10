use anyhow::Error;

#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error("parse error {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("io error {0}")]
    IOError(#[from] std::io::Error),

    #[error("missing parameters")]
    MissingParameters,

    #[error("not found")]
    NotFound,

    #[error("internal error {0}")]
    InternalError(#[from] Error),
}
