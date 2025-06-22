use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Processor(#[from] intel8080_core::errors::Error),

    #[error("SDL window rendering failed:\n{0}")]
    Sdl(String),

    #[error("File IO failed:\n{0}")]
    IO(#[from] std::io::Error),

    #[error("Invalid DIP switch input")]
    InvalidDipInput,

    #[error("Audio output failed:\n{0}")]
    AudioStream(#[from] rodio::StreamError),
    
    #[error("Audio output failed:\n{0}")]
    AudioPlay(#[from] rodio::PlayError),
    
    #[error("Audio output failed:\n{0}")]
    AudioDecoder(#[from] rodio::decoder::DecoderError),
}

pub type Result<T> = std::result::Result<T, Error>;
