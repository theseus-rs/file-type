/// File type result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur when determining the file type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An I/O error occurred
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// An error occurred when parsing a byte sequence regex
    #[error("{0}")]
    Syntax(String),
    /// The file type is unknown
    #[error("The file type is unknown")]
    UnknownFileType,
}
