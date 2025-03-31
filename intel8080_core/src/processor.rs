use crate::{memory::Memory, errors::{Result, Error}};

pub struct Processor {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    ram: Memory,
    rom_loaded: bool,
    flags: Flags,
}

struct Flags {
    pub s: bool,
    pub z: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
}

impl Processor {
    pub fn new(ram_size: usize, memory_mapper: fn(u16) -> usize) -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            ram: Memory::new(ram_size, memory_mapper),
            rom_loaded: false,
            flags: Flags {
                s: false,
                z: false,
                p: false,
                cy: false,
                ac: false,
            },
        }
    }

    pub fn execute(&mut self) -> Result<u32> {
        let opcode: u8 = self.ram.read(self.pc)?;
        let mut cycles: u32 = 0;

        match opcode {
            // NOP opcodes
            0x00 | 0x20 | 0x30 => {}

            // HLT opcode
            0x76 => return Err(Error::SystemHalt),

            // MOV Register to register
            0x40..0x80 => {
                cycles = self.mov_opcode(opcode)?;
                self.pc += 1;
            }
            
            // MVI opcodes
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                let data = self.ram.read(self.pc + 1)?;
                self.mvi_opcode(opcode, data)?
            }

            _ => return Err(Error::UnimplementedOpcodeError(opcode))
        }

        self.pc += 1;

        Ok(cycles)
    }
    
    fn mov_opcode(&mut self, opcode: u8) -> Result<u32> {
        let (source, from_memory) = self.get_source_reg(opcode & 0b111)?;
        
        let destination = self.get_dest_reg((opcode >> 3) & 0b111)?;

        *destination = source;

        let cycles = match from_memory {
            true => 7,
            false => 5,
        };
        Ok(cycles)
    }

    fn mvi_opcode(&mut self, opcode: u8, data: u8) -> Result<()> {
        let destination = self.get_dest_reg((opcode >> 3) & 0b111)?;
        *destination = data;

        Ok(())
    }

    fn get_source_reg(&mut self, reg: u8) -> Result<(u8, bool)> {
        let mut from_memory = false;
        let source = match reg {
            0b111 => self.a,
            0b000 => self.b,
            0b001 => self.c,
            0b010 => self.d,
            0b011 => self.e,
            0b100 => self.h,
            0b101 => self.l,
            0b110 => {
                from_memory = true;
                self.ram.read(self.get_hl())?
            }
            _ => panic!("Failed to parse register {:#b}", reg),
        };

        Ok((source, from_memory))
    }

    fn get_dest_reg(&mut self, reg: u8) -> Result<&mut u8> {
        // TODO Change to a setter function
        let destination = match reg {
            0b111 => &mut self.a,
            0b000 => &mut self.b,
            0b001 => &mut self.c,
            0b010 => &mut self.d,
            0b011 => &mut self.e,
            0b100 => &mut self.h,
            0b101 => &mut self.l,
            0b110 => self.ram.read_mut(self.get_hl())?,
            _ => panic!("Failed to parse register in opcode: {:#b}", reg),
        };
        Ok(destination)
    }
    
    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) & self.c as u16
    }

    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) & self.e as u16
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) &self.l as u16
    }


}
