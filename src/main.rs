#![allow(dead_code)]

use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use Register::*;

// starts at 0 (EAX=0)
enum Register { EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI, RegisterCount }

struct Emulator {
    // General-purpose Registers
    registers: [u32; RegisterCount as usize],
    // EFLAGS Register
    eflags: u32,
    // Instruction Pointer
    eip: u32,
    // Memory
    memory: Vec<u8>
}

fn create_emu() -> Emulator {
    // create new instance
    Emulator {
        registers: [0; RegisterCount as usize],
        eflags: 0,
        eip: 0,
        memory: Vec::new()
    }
}

fn main() {
    // read cmdline args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: x86_emu filename");
        process::exit(1);
    }
    let filename = &args[1];

    // open file
    let mut file = File::open(filename)
        .expect("file not found");

    // read file as byte
    let mut emu = create_emu();
    file.read_to_end(&mut emu.memory)
        .expect("something went wrong reading the file");

    println!("Contents:\n {:02x?}", &emu.memory);
}
