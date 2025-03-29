use std::fs;

use crate::errors::Error;

pub fn disassemble(rom_path: &str, output: Option<String>) -> Result<(), Error> {
    let rom: Vec<u8> = fs::read(rom_path)?;
    let length = rom.len();

    let lines: Vec<String> = Vec::new();

    let mut i = 0;
    while i < length {
        let opcode = rom[i];

        let line = match opcode {
            0x00 => format_op(i, "NOP", opcode, None, None, None),
            0x01 => {
                let low = rom[i + 1];
                let high = rom[i + 1];
                let comment = format!("Load {high:#02X}{low:02X} into B");
                format_op(i, "LXI B,", opcode, Some(high), Some(low), Some(&comment))
            }
            0x02 => format_op(i, "STAX B", opcode, None, None, Some("Load A to BC")),
            0x03 => format_op(i, "INX B", opcode, None, None, Some("Increment BC")),
            0x04 => format_op(i, "INR B", opcode, None, None, Some("Increment B - flags: Z, S, P, AC")),
            0x05 => format_op(i, "DCR B", opcode, None, None, Some("Decrement B")),
            0x06 => {
                let high = rom[i + 1];
                let comment = format!("Load {high:02X}");
                format_op(i, "MVI B,", opcode, Some(high), None, Some(&comment))
            }
        };
    }

    todo!()
}

fn format_op(
    i: usize,
    mnemonic: &str,
    opcode: u8,
    high: Option<u8>,
    low: Option<u8>,
    comment: Option<&str>,
) -> String {
    match (high, low, comment) {
        (Some(high), Some(low), Some(comment)) => {
            format!("{i:04X}    {opcode:02X}  {high:02X}  {low:02X}    {mnemonic:<4} {high:#02X}{low:02X} ; {comment}")
        }

        (Some(high), Some(low), None) => {
            format!("{i:04X}    {opcode:02X}  {high:02X}  {low:02X}    {mnemonic:<4} {high:#02X}{low:02X}")
        },

        (Some(high), None, None) => {
            format!("{i:04X}    {opcode:02X}  {high:02X}        {mnemonic} {high:#02X}")
        }

        (Some(high), None, Some(comment)) => {
            format!("{i:04X}    {opcode:02X}  {high:02X}        {mnemonic} {high:#02X} ; {comment}")
        }

        (None, None, Some(comment)) => {
            format!("{i:04X}    {opcode:02X}            {mnemonic:<8} ; {comment}")
        },

        (None, None, None) => {
            format!("{i:04X}    {opcode:02X}            {mnemonic}")
        },

        _ => String::new(),
    }
}
