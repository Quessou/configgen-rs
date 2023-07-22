use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration directory already exists")]
    ConfigDirectoryAlreadyExists(#[source] std::io::Error),
    #[error("Unhandled serialization format")]
    UnsupportedFormat(#[source] std::io::Error),
    #[error("Unhandled serialization format")]
    SerializationFailed(#[source] Box<dyn std::error::Error>),
    #[error("Writing failed")]
    WritingFailed(#[source] std::io::Error),
}
