use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    EmulatorError(#[from] intel8080_core::errors::Error),

    #[error("SDL window rendering failed:\n{0}")]
    SdlError(String),

    #[error("File IO failed:\n{0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid DIP switch input")]
    InvalidDipInput,

    #[error("Audio output failed:\n{0}")]
    AudioStreamError(#[from] rodio::StreamError),
    
    #[error("Audio output failed:\n{0}")]
    AudioPlayError(#[from] rodio::PlayError),
    
    #[error("Audio output failed:\n{0}")]
    AudioDecoderError(#[from] rodio::decoder::DecoderError),
}

pub type Result<T> = std::result::Result<T, Error>;
