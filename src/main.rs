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

fn create_emu(eip: u32, esp: u32) -> Emulator {
    let mut emu = Emulator {
        // Clear all resisters by 0
        registers: [0; RegisterCount as usize],
        // Clear eflags by 0
        eflags: 0,
        // Init EIP register
        eip: eip,
        // Init memory
        memory: Vec::new()
    };

    // Init ESP register
    emu.registers[ESP as usize] = esp;

    emu
}

fn main() {
    // Read filename from command args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: x86_emu filename");
        process::exit(1);
    }
    let filename = &args[1];

    // Open binary file
    let mut file = File::open(filename)
        .expect("file not found");

    // Create emulator with EIP=0x0000 and ESP=0x7c00
    let mut emu = create_emu(0x0000, 0x7c00);
    
    // Read binary file into memory
    file.read_to_end(&mut emu.memory)
        .expect("something went wrong reading the file");

    println!("Contents:\n {:02x?}", &emu.memory);
}
