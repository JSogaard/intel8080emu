use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),
    
    #[error("Found unknown opcode: {0:#02X}")]
    UnknownOpcodeError(u8)
}