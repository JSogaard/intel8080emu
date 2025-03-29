use std::fs;

use crate::errors::Error;

pub fn disassemble(rom_path: &str, output: Option<String>) -> Result<(), Error> {
    let rom: Vec<u8> = fs::read(rom_path)?;
    let length = rom.len();

    let lines: Vec<String> = Vec::new();

    let mut i = 0;
    while i < length {
        let opcode = rom[i];

        let line: String = match opcode {
            0x00 | 0x20 | 0x30 => format_no_arg(i, "NOP", opcode, ""),
            0x01 => {
                let low = rom[i + 1];
                let high = rom[i + 1];
                let comment = format!("Load {high:#02X}{low:02X} into BC");
                i += 2;
                format_two_args(i, "LXI B,", opcode, high, low, &comment)
            }
            0x02 => format_no_arg(i, "STAX B", opcode, "Store A at (BC)"),
            0x03 => format_no_arg(i, "INX B", opcode, "Increment BC"),
            0x04 => format_no_arg(i, "INR B", opcode, "Increment B - flags: Z, S, P, AC"),
            0x05 => format_no_arg(i, "DCR B", opcode, "Decrement B - flags: Z, S, P, AC"),
            0x06 => {
                let byte = rom[i + 1];
                let comment = format!("Load literal {byte:#02X} into B");
                i += 1;
                format_one_arg(i, "MVI B,", opcode, byte, &comment)
            }
            0x07 => format_no_arg(
                i,
                "RLC",
                opcode,
                "A << 1, bit 0 = prev bit 7, CY = prev bit 7",
            ),
            0x09 => format_no_arg(i, "DAD B", opcode, "HL += BC"),
            0x0A => format_no_arg(i, "LDAX B", opcode, "Load (BC) into A"),
            0x0B => format_no_arg(i, "DCX B", opcode, "Decrement BC"),
            0x0C => format_no_arg(i, "INR C", opcode, "Increment C - flags: Z, S, P, AC"),
            0x0D => format_no_arg(i, "DCR C", opcode, "Decrement C - flags: Z, S, P, AC"),
            0x0E => {
                let byte = rom[i + 1];
                let comment = format!("Load literal {byte:#02X}");
                format_one_arg(i, "mnemonic", opcode, byte, &comment)
            }
            0x0F => format_no_arg(i, "RRC", opcode, "A >> 1, bit 7 = prev bit 0, CY = prev bit 0"),
            0x11 => {
                let low = rom[i + 1];
                let high = rom[i + 2];
                let comment = format!("Load {high:#02X}{low:02X} into DE");
                format_two_args(i, "LXI D,", opcode, high, low, &comment)
            }
            0x12 => format_no_arg(i, "STAX D", opcode, "Store A at (DE)"),
            0x13 => format_no_arg(i, "INX D", opcode, "Increment DE"),
            0x14 => format_no_arg(i, "INR D", opcode, "Increment D - flags: Z, S, P, AC"),
            0x15 => format_no_arg(i, "DCR D", opcode, "Decrement D - flags: Z, S, P, AC"),
            0x16 => {
                let byte = rom[i + 1];
                let comment = format!("Load literal {byte:#02X} onto D");
                format_one_arg(i, "MVI D,", opcode, byte, &comment)
            }
            0x17 => format_no_arg(i, "RAL", opcode, "A << 1, bit 0 = prev CY, CY = prev bit 7"),
            0x19 => format_no_arg(i, "DAD D", opcode, "HL += DE"),
            0x1A => format_no_arg(i, "LDAX D", opcode, "Load (DE) into A"),
            0x1B => format_no_arg(i, "DCX D", opcode, "Decrement DE"),
            0x1C => format_no_arg(i, "INR E", opcode, "Increment C - flags: Z, S, P, AC"),
            0x1D => format_no_arg(i, "DCR E", opcode, "Decrement E - flags: Z, S, P, AC"),
            0x1E => {
                let byte = rom[i + 1];
                let comment = format!("Load literal {byte:#02X} onto E");
                format_one_arg(i, "MVI E,", opcode, byte, &comment)
            }
            0x1F => format_no_arg(i, "RAR", opcode, "A >> 1, bit 7 = prev bit 7, CY = prev bit 0"),

        };
        i += 1;
    }
    // TODO Disassembler: Join string and print or save to file

    todo!()
}

fn format_no_arg(i: usize, mnemonic: &str, opcode: u8, comment: &str) -> String {
    format!("{i:04X}    {opcode:02X}            {mnemonic:<6} ; {comment}")
}

fn format_one_arg(i: usize, mnemonic: &str, opcode: u8, byte: u8, comment: &str) -> String {
    format!("{i:04X}    {opcode:02X}  {byte:02X}        {mnemonic:<6} {byte:#02X} ; {comment}")
}

fn format_two_args(
    i: usize,
    mnemonic: &str,
    opcode: u8,
    high: u8,
    low: u8,
    comment: &str,
) -> String {
    format!(
        "{i:04X}    {opcode:02X}  {high:02X}  {low:02X}    {mnemonic:<6} {high:#02X}{low:02X} ; {comment}"
    )
}
