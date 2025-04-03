pub fn bytes_to_16bit(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}

pub fn bit_parity(byte: u8) -> bool {
    let mut bits = 0;
    for i in 0..8 {
        bits += (byte >> i) & 1;
    }

    bits % 2 == 0
}

pub fn auxiliary_add(a: u8, b: u8) -> bool {
    (a & 0xF) + (b & 0xF) > 0xF
}

pub fn auxiliary_sub(a: u8, b: u8) -> bool {
    (a & 0xF) < (b & 0xF)
}