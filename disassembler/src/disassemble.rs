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
            0x00 => format_instruction(i, "NOP", opcode, None, None, None),
            0x01 => {
                let low = rom[i + 1];
                let high = rom[i + 1];
                let mnemonic = "LXI";
                let comment = format!("Load {high:#02X}{low:02X} into B");
                format_instruction(i, mnemonic, opcode, Some(high), Some(low), Some(&comment))
            }
        };
    }

    todo!()
}

fn format_instruction(
    i: usize,
    mnemonic: &str,
    opcode: u8,
    high: Option<u8>,
    low: Option<u8>,
    comment: Option<&str>,
) -> String {
    match (high, low, comment) {
        (Some(high), Some(low), Some(comment)) => {
            format!("{i:04X}    {opcode:02X}  {high:02X}  {low:02X}    {mnemonic:<8}; {comment}")
        },

        (Some(high), Some(low), None) => {
            format!("{i:04X}    {opcode:02X}  {high:02X}  {low:02X}    {mnemonic}")
        },

        (None, None, Some(comment)) => {
            format!("{i:04X}    {opcode:02X}            {mnemonic:<8}; {comment}")
        },

        (None, None, None) => {
            format!("{i:04X}    {opcode:02X}            {mnemonic}")
        }

        _ => {String::new()},
    }
}
