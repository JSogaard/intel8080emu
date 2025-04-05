use crate::{
    errors::{Error, Result},
    helpers::{auxiliary_add, auxiliary_sub, bit_parity, bytes_to_16bit},
    memory::Memory,
};

#[derive(Clone, Debug)]
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
    interrupts_enabled: bool,
    flags: Flags,
}

#[derive(Clone, Debug)]
struct Flags {
    s: bool,
    z: bool,
    p: bool,
    cy: bool,
    ac: bool,
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
            interrupts_enabled: false,
            flags: Flags {
                s: false,
                z: false,
                p: false,
                cy: false,
                ac: false,
            },
        }
    }

    pub fn load_rom(&mut self, rom: &[u8], address: u16) -> Result<()> {
        self.ram.load_rom(rom, address)?;
        self.rom_loaded = true;

        Ok(())
    }

    pub fn execute(&mut self) -> Result<u32> {
        if !self.rom_loaded {
            return Err(Error::RomNotLoaded);
        }

        let opcode: u8 = self.ram.read(self.pc)?;
        let cycles: u32;

        match opcode {
            // NOP opcodes
            0x00 => {
                self.pc += 1;
                cycles = 4;
            }

            // HLT opcode
            0x76 => return Err(Error::SystemHalt),

            // MOV Register to register
            0x40..=0x7F => {
                cycles = self.mov_opcode(opcode)?;
                self.pc += 1;
            }

            // MVI opcodes
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                let data = self.ram.read(self.pc + 1)?;
                self.mvi_opcode(opcode, data)?;
                self.pc += 2;
                cycles = 7;
            }

            // LXI opcodes
            0x01 | 0x11 | 0x21 | 0x31 => {
                let (low_byte, high_byte) = self.get_next_16bit()?;
                self.lxi_opcode(opcode, low_byte, high_byte);
                self.pc += 3;
                cycles = 10;
            }

            // LDA opcode
            0x3A => {
                let (low_byte, high_byte) = self.get_next_16bit()?;
                self.lda_opcode(low_byte, high_byte)?;
                self.pc += 3;
                cycles = 13;
            }

            // STA opcode
            0x32 => {
                let (low_byte, high_byte) = self.get_next_16bit()?;
                self.sta_opcode(low_byte, high_byte)?;
                self.pc += 3;
                cycles = 13;
            }

            // LHLD opcode
            0x2A => {
                let (low_byte, high_byte) = self.get_next_16bit()?;
                self.lhld_opcode(low_byte, high_byte)?;
                self.pc += 3;
                cycles = 16;
            }

            // SHLD opcode
            0x22 => {
                let (low_byte, high_byte) = self.get_next_16bit()?;
                self.shld_opcode(low_byte, high_byte)?;
                self.pc += 3;
                cycles = 16;
            }

            // LDAX opcode
            0x0A | 0x1A => {
                self.ldax_opcode(opcode)?;
                self.pc += 1;
                cycles = 7;
            }

            // Stax
            0x02 | 0x12 => {
                self.stax_opcode(opcode)?;
                self.pc += 1;
                cycles = 7;
            }

            // XCHG opcode
            0xEB => {
                self.xchg_opcode();
                self.pc += 1;
                cycles = 5;
            }

            // ADD opcodes
            0x80..=0x87 => {
                cycles = self.add_opcode(opcode)?;
                self.pc += 1;
            }

            // ADI opcode
            0xC6 => {
                let immediate = self.ram.read(self.pc + 1)?;
                self.adi_opcode(immediate)?;
                self.pc += 2;
                cycles = 7;
            }

            // ADC opcodes
            0x88..=0x8F => {
                cycles = self.adc_opcode(opcode)?;
                self.pc += 1;
            }

            // ACI opcode
            0xCE => {
                let immediate = self.ram.read(self.pc + 1)?;
                self.aci_opcode(immediate)?;
                self.pc += 2;
                cycles = 7;
            }

            // Sub opcodes
            0x90..=0x97 => {
                cycles = self.sub_opcode(opcode)?;
                self.pc += 1;
            }

            // SUI opcode
            0xD6 => {
                let immediate = self.ram.read(self.pc + 1)?;
                self.sui_opcode(immediate)?;
                self.pc += 2;
                cycles = 7;
            }

            // SBB Opcodes
            0x98..=0x9F => {
                cycles = self.sbb_opcode(opcode)?;
                self.pc += 1;
            }

            // Invalid opcodes
            0x10 | 0x20 | 0x30 | 0x08 | 0x18 | 0x28 | 0x38 | 0xD9 | 0xCB | 0xDD | 0xED | 0xFD => {
                return Err(Error::UnknownOpcode(opcode));
            }
        }

        Ok(cycles)
    }

    fn get_source_reg(&mut self, opcode: u8) -> Result<(u8, bool)> {
        let mut from_memory = false;
        let source = match opcode & 0b111 {
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
            _ => panic!("Failed to parse register {:#b}", opcode & 0b111),
        };

        Ok((source, from_memory))
    }

    fn set_dest_reg(&mut self, opcode: u8, data: u8) -> Result<()> {
        let destination: &mut u8 = match (opcode >> 3) & 0b111 {
            0b111 => &mut self.a,
            0b000 => &mut self.b,
            0b001 => &mut self.c,
            0b010 => &mut self.d,
            0b011 => &mut self.e,
            0b100 => &mut self.h,
            0b101 => &mut self.l,
            0b110 => self.ram.read_mut(self.get_hl())?,
            _ => panic!("Failed to parse register: {:#b}", (opcode >> 3) & 0b111),
        };
        *destination = data;

        Ok(())
    }

    fn set_reg_pair(&mut self, opcode: u8, low_byte: u8, high_byte: u8) {
        match (opcode >> 4) & 0b11 {
            0b00 => (self.b, self.c) = (high_byte, low_byte),
            0b01 => (self.d, self.e) = (high_byte, low_byte),
            0b10 => (self.h, self.l) = (high_byte, low_byte),
            0b11 => self.sp = bytes_to_16bit(low_byte, high_byte),
            _ => panic!("Failed to parse registerpair: {:#b}", (opcode >> 4) & 0b11),
        }
    }

    fn get_reg_pair(&self, opcode: u8) -> u16 {
        match (opcode >> 4) & 0b11 {
            0b00 => bytes_to_16bit(self.c, self.b),
            0b01 => bytes_to_16bit(self.e, self.d),
            0b10 => bytes_to_16bit(self.l, self.h),
            0b11 => self.sp,
            _ => panic!("Failed to parse registerpair: {:#b}", (opcode >> 4) & 0b11),
        }
    }

    fn get_bc(&self) -> u16 {
        bytes_to_16bit(self.c, self.b)
    }

    fn get_de(&self) -> u16 {
        bytes_to_16bit(self.e, self.d)
    }

    fn get_hl(&self) -> u16 {
        bytes_to_16bit(self.l, self.h)
    }

    fn get_next_16bit(&self) -> Result<(u8, u8)> {
        let low_byte = self.ram.read(self.pc + 1)?;
        let high_byte = self.ram.read(self.pc + 2)?;
        Ok((low_byte, high_byte))
    }

    fn set_flags_add(&mut self, result_16: u16, result_8: u8, prev_a: u8, b: u8) {
        self.flags.s = ((result_8 >> 7) & 1) == 1;
        self.flags.z = result_8 == 0;
        self.flags.p = bit_parity(result_8);
        self.flags.cy = result_16 & 0xFF00 != 0;
        self.flags.ac = auxiliary_add(prev_a, b);
    }

    fn set_flags_sub(&mut self, result: u8, a: u8, b: u8) {
        self.flags.s = ((result >> 7) & 1) == 1;
        self.flags.z = result == 0;
        self.flags.p = bit_parity(result);
        self.flags.cy = a < b;
        self.flags.ac = auxiliary_sub(a, b);
    }

    // =====================================================================
    //                            OPCODE FUNCTIONS
    // =====================================================================

    fn mov_opcode(&mut self, opcode: u8) -> Result<u32> {
        let (source, from_memory) = self.get_source_reg(opcode)?;

        self.set_dest_reg(opcode, source)?;

        if from_memory { Ok(7) } else { Ok(5) }
    }

    fn mvi_opcode(&mut self, opcode: u8, immediate: u8) -> Result<()> {
        self.set_dest_reg(opcode, immediate)?;

        Ok(())
    }

    fn lxi_opcode(&mut self, opcode: u8, low_byte: u8, high_byte: u8) {
        self.set_reg_pair(opcode, low_byte, high_byte);
    }

    fn lda_opcode(&mut self, low_byte: u8, high_byte: u8) -> Result<()> {
        let address = bytes_to_16bit(low_byte, high_byte);
        self.a = self.ram.read(address)?;

        Ok(())
    }

    fn sta_opcode(&mut self, low_byte: u8, high_byte: u8) -> Result<()> {
        let address = bytes_to_16bit(low_byte, high_byte);
        self.ram.write(address, self.a)?;

        Ok(())
    }

    fn lhld_opcode(&mut self, low_byte: u8, high_byte: u8) -> Result<()> {
        let address = bytes_to_16bit(low_byte, high_byte);
        self.l = self.ram.read(address)?;
        self.h = self.ram.read(address + 1)?;

        Ok(())
    }

    fn shld_opcode(&mut self, low_byte: u8, high_byte: u8) -> Result<()> {
        let address = bytes_to_16bit(low_byte, high_byte);
        self.ram.write(address, self.l)?;
        self.ram.write(address + 1, self.h)?;

        Ok(())
    }

    fn ldax_opcode(&mut self, opcode: u8) -> Result<()> {
        let address = self.get_reg_pair(opcode);
        self.a = self.ram.read(address)?;

        Ok(())
    }

    fn stax_opcode(&mut self, opcode: u8) -> Result<()> {
        let address = self.get_reg_pair(opcode);
        self.ram.write(address, self.a)?;

        Ok(())
    }

    fn xchg_opcode(&mut self) {
        let d_prev = self.d;
        let e_prev = self.e;
        self.d = self.h;
        self.e = self.l;
        self.h = d_prev;
        self.l = e_prev;
    }

    fn add_opcode(&mut self, opcode: u8) -> Result<u32> {
        let (source, from_memory) = self.get_source_reg(opcode)?;
        let result = self.a as u16 + source as u16;
        let prev_a = self.a;
        self.a = (result & 0xFF) as u8;

        self.set_flags_add(result, self.a, prev_a, source);

        // Cycles count depends on whether or not memory was accessed
        if from_memory { Ok(7) } else { Ok(4) }
    }

    fn adi_opcode(&mut self, immediate: u8) -> Result<()> {
        let result = self.a as u16 + immediate as u16;
        let prev_a = self.a;
        self.a = (result & 0xFF) as u8;

        self.set_flags_add(result, self.a, prev_a, immediate);

        Ok(())
    }

    fn adc_opcode(&mut self, opcode: u8) -> Result<u32> {
        let (source, from_memory) = self.get_source_reg(opcode)?;
        let result = self.a as u16 + source as u16 + self.flags.cy as u16;
        let prev_a = self.a;
        self.a = (result & 0xFF) as u8;

        self.set_flags_add(result, self.a, prev_a, source + self.flags.cy as u8);

        // Cycles count depends on whether or not memory was accessed
        if from_memory { Ok(7) } else { Ok(4) }
    }

    fn aci_opcode(&mut self, immediate: u8) -> Result<()> {
        let result = self.a as u16 + immediate as u16 + self.flags.cy as u16;
        let prev_a = self.a;
        self.a = (result & 0xFF) as u8;

        self.set_flags_add(result, self.a, prev_a, immediate + self.flags.cy as u8);

        Ok(())
    }

    fn sub_opcode(&mut self, opcode: u8) -> Result<u32> {
        let (source, from_memory) = self.get_source_reg(opcode)?;
        let prev_a = self.a;
        self.a = self.a.wrapping_sub(source);

        self.set_flags_sub(self.a, prev_a, source);

        // Cycles count depends on whether or not memory was accessed
        if from_memory { Ok(7) } else { Ok(4) }
    }

    fn sui_opcode(&mut self, immediate: u8) -> Result<()> {
        let prev_a = self.a;
        self.a = self.a.wrapping_sub(immediate);

        self.set_flags_sub(self.a, prev_a, immediate);

        Ok(())
    }

    fn sbb_opcode(&mut self, opcode: u8) -> Result<u32> {
        let (source, from_memory) = self.get_source_reg(opcode)?;
        let prev_a = self.a;
        self.a = self
            .a
            .wrapping_sub(source)
            .wrapping_sub(self.flags.cy as u8);

        self.set_flags_sub(self.a, prev_a, source + self.flags.cy as u8);

        if from_memory { Ok(7) } else { Ok(4) }
    }
}
