// #![allow(warnings)]

use clap::Parser;
use clap_num::maybe_hex;

pub mod main_emulator;
pub mod cpu_state;
pub mod instructions;

use crate::main_emulator::*;
use crate::cpu_state::*;

/// Intel 8080 CPU emulator / runtime simulator in Rust
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Name of a ROM file to load
    #[arg(short, long)]
    file: String,
    /// Memory address to load the ROM file at in hex
    #[arg(short, long, value_parser=maybe_hex::<usize>, default_value="0")]
    address: usize,
    /// e for emulator s for simulator
    #[arg(short, long, default_value = "e")]
    mode: String,
}

fn main() {
    let args = Args::parse();

    println!("file: {}, mem_addr: {}", args.file, args.address);

    let state = &mut State::new();
    let file = args.file;
    let mem_addr = args.address;
    read_file_to_memory(state, &file, mem_addr);

    loop {
        emulate8080(state);
    }
}
