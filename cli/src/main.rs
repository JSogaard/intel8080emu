use clap::{Parser, Subcommand};

fn main() {
    todo!()
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