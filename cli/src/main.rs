use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    todo!();

    Ok(())
}

#[derive(Parser)]
#[command(version)]
/// An Intel 8080 emulator, implemented in Rust.
struct Cli {
    rom_path: String,
}