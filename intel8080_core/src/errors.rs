use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid memory address was accesed")]
    InvalidMemoryError,

    #[error("Unimplemented opcode found: {0}")]
    UnimplementedOpcodeError(u8),
}

pub type Result<T> = std::result::Result<T, Error>;