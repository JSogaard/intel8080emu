use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("SDL window rendering failed:\n{0}")]
    SdlError(String),

    #[error("File IO failed:\n{0}")]
    IoError(#[from] std::io::Error),
}