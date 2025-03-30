use crate::errors::{Error, Result};

pub struct Memory {
    data: Vec<u8>,
    size: usize,
    memory_mapper: fn(u16) -> usize,
}

impl Memory {
    pub fn new(size: usize, memory_mapper: fn(u16) -> usize) -> Self {
        Self {
            data: vec![0; size],
            size,
            memory_mapper,
        }
    }

    pub fn read(&self, address: u16) -> Result<u8> {
        let address = (self.memory_mapper)(address);
        if address >= self.size {
            return Err(Error::InvalidMemoryError)
        }
        Ok(self.data[address])
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<()> {
        let address = (self.memory_mapper)(address);
        if address >= self.size {
            return Err(Error::InvalidMemoryError)
        }
        self.data[address] = value;
        
        Ok(())
    }
}