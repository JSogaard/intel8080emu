use anyhow::Result;
use clap::{Parser, Subcommand};
use disassembler::disassemble::disassemble;

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Disassemble { rom_path, output } => {
            disassemble(&rom_path, output)?
        }
    }

    Ok(())
}

#[derive(Parser)]
#[command(version)]
/// An Intel 8080 emulator, implemented in Rust.
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Disassemble ROM
    Disassemble {
        rom_path: String,
        #[arg(short, long)]
        output: Option<String>,
    },
}