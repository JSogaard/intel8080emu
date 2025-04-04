pub fn bytes_to_16bit(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}

/// Checks if the number of 1s in byte is even
pub fn bit_parity(byte: u8) -> bool {
    let mut byte = byte;

    let mut shift = 4;
    while shift > 0 {
        let mask = 0xFF >> (8 - shift);
        byte = (byte & mask) ^ (byte >> shift);
        shift /= 2;
    }
    byte == 0
}

pub fn auxiliary_add(a: u8, b: u8) -> bool {
    (a & 0xF) + (b & 0xF) > 0xF
}

pub fn auxiliary_sub(a: u8, b: u8) -> bool {
    (a & 0xF) < (b & 0xF)
}