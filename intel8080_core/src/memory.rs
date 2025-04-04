use crate::errors::{Error, Result};

#[derive(Clone, Debug)]
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

    pub fn load_rom(&mut self, rom: &[u8], address: u16) -> Result<()> {
        let address = (self.memory_mapper)(address);

        if rom.len() > self.size - address {
            return Err(Error::RomSize {
                rom_size: rom.len(),
                space_left: self.size - address,
            });
        }
        self.data[address..address + rom.len()].copy_from_slice(rom);

        Ok(())
    }

    pub fn read(&self, address: u16) -> Result<u8> {
        let address = (self.memory_mapper)(address);
        if address >= self.size {
            return Err(Error::InvalidMemory(address));
        }
        Ok(self.data[address])
    }

    pub fn read_mut(&mut self, address: u16) -> Result<&mut u8> {
        let address = (self.memory_mapper)(address);
        if address >= self.size {
            return Err(Error::InvalidMemory(address));
        }
        Ok(&mut self.data[address])
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<()> {
        let address = (self.memory_mapper)(address);
        if address >= self.size {
            return Err(Error::InvalidMemory(address));
        }
        self.data[address] = value;

        Ok(())
    }
}
