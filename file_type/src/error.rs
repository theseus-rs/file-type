/// File type result type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors that can occur when determining the file type
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    message: String,
}

impl Error {
    pub(crate) fn new<S: AsRef<str>>(message: S) -> Error {
        Error {
            message: message.as_ref().to_string(),
        }
    }
}

impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let error = Error::new("test");
        assert_eq!(error.to_string(), "test");
    }
}
