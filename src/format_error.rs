use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Inconsistent Row Width")]
    InconsistentRowWidth,
    #[error("Write Error: {0}")]
    WriteError(String),
}