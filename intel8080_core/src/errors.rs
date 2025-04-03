use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid memory address was accesed: {0}")]
    InvalidMemory(usize),

    #[error("Unknown opcode found: {0}")]
    UnknownOpcode(u8),

    #[error("Failed to parse register: {0}")]
    RegisterParse(u8),

    #[error("Program was halted by opcode")]
    SystemHalt,

    #[error("ROM is {rom_size} bytes but there is only {space_left} after target address")]
    RomSize {
        rom_size: usize,
        space_left: usize,
    },

    #[error("No ROM has been loaded")]
    RomNotLoaded,
}

pub type Result<T> = std::result::Result<T, Error>;