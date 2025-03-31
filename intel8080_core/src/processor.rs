use self::{Reg8bit::*, Reg16bit::*};
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
            // Nop opcodes
            0x00 | 0x20 | 0x30 => {}

            // MOV Register to register
            0x40..0x70 if !matches!(opcode & 0xF, 0x6 | 0xE) => {
                self.mov_registers(opcode);
                self.pc += 1;
                cycles = 5;
            }
            

            _ => return Err(Error::UnimplementedOpcodeError(opcode))
        }

        self.pc += 1;

        Ok(cycles)
    }

    fn get_reg(&self, reg: u8) -> Option<u8> {
        match reg {
            0b111 => Some(self.a),

        }
    }

    fn mov_registers(&mut self, opcode: u8) {
        // TODO MOV registers

        let dest_reg = (opcode >> 3) & 0b111;
        let source_reg = opcode & 0b111;

        todo!()
    }
}

enum Reg8bit {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum Reg16bit {
    BC,
    DE,
    HL,
}