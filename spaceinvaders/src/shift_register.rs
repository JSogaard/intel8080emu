#[derive(Clone, Debug, Default)]
pub struct ShiftRegister {
    register: u16,
    offset: u8,
}

impl ShiftRegister {
    pub fn new() -> Self {
        Self {
            register: 0,
            offset: 0,
        }
    }

    /// Insert value into high byte on OUT 4
    pub fn insert(&mut self, value: u8) {
        self.register = ((value as u16) << 8) | (self.register >> 8);
    }

    /// Set offset on OUT 2
    pub fn set_offset(&mut self, offset: u8) {
        self.offset = offset & 0b111;
    }

    /// Read high byte shifted by offset on IN 3
    pub fn read(&self) -> u8 {
        let shift = 8 - self.offset as u16;
        (self.register >> shift) as u8
    }
}