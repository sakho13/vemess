use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("invalid parameter: {0}")]
    ValidationError(String),
    #[error("parse error: {0}")]
    ParseError(String),
}
