use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration directory already exists")]
    ConfigDirectoryAlreadyExists(#[source] std::io::Error),
    #[error("Configuration file already exists")]
    ConfigFileAlreadyExists(#[source] std::io::Error),
    #[error("Configuration directory creation failed")]
    ConfigDirectoryCreationFailed(#[source] std::io::Error),
    #[error("Unhandled serialization format")]
    UnsupportedFormat(#[source] std::io::Error),
    #[error("Unhandled serialization format")]
    SerializationFailed(#[source] Box<dyn std::error::Error>),
    #[error("Writing failed")]
    WritingFailed(#[source] std::io::Error),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (
                Self::ConfigDirectoryAlreadyExists(_),
                Self::ConfigDirectoryAlreadyExists(_)
            ) | (
                Self::ConfigFileAlreadyExists(_),
                Self::ConfigFileAlreadyExists(_)
            ) | (
                Self::ConfigDirectoryCreationFailed(_),
                Self::ConfigDirectoryCreationFailed(_)
            ) | (Self::UnsupportedFormat(_), Self::UnsupportedFormat(_))
                | (Self::SerializationFailed(_), Self::SerializationFailed(_))
                | (Self::WritingFailed(_), Self::WritingFailed(_))
        )
    }
}

#[cfg(test)]
mod tests {
    use std::assert_ne;

    use super::*;

    #[test]
    pub fn test_error_comparison() {
        let err1 = Error::ConfigFileAlreadyExists(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "",
        ));
        let err2 = Error::ConfigFileAlreadyExists(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "",
        ));
        let err3 = Error::ConfigDirectoryCreationFailed(std::io::Error::new(
            std::io::ErrorKind::Other,
            "",
        ));

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }
}
