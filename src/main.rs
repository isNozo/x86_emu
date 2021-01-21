#![allow(dead_code)]

use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use Register::*;

// General-purpose Registers Set
// This enum is used as index to access the registers[] field of Emulator
enum Register { EAX=0, ECX, EDX, EBX, ESP, EBP, ESI, EDI }
const REGISTERS_COUNT: usize = 8;
const REGISTERS_NAME: [&str; REGISTERS_COUNT] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

struct Emulator {
    // General-purpose Registers
    registers: [u32; REGISTERS_COUNT],
    // EFLAGS Register
    eflags: u32,
    // Instruction Pointer
    eip: u32,
    // Memory
    memory: Vec<u8>
}

// Create a new instance of the emulator with EIP and ESP
fn create_emu(eip: u32, esp: u32) -> Emulator {
    let mut emu = Emulator {
        // Clear all resisters by 0
        registers: [0; REGISTERS_COUNT],
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

// Read a byte code from a emulator's memory
fn get_code8(emu: &Emulator, offset: usize) -> u8 {
    emu.memory[emu.eip as usize + offset]
}

// Initialize a instructions table
const INSTRUCTIONS_COUNT: usize = 256;
fn init_instructions(instructions: &mut [Option<fn(&mut Emulator)>; INSTRUCTIONS_COUNT]) {
    instructions[0] = None;
}

// Dump general-purpose registers and EIP values
fn dump_registers(emu: &Emulator) {
    for i in 0..REGISTERS_COUNT {
        println!("{} = {:#010x}", REGISTERS_NAME[i], emu.registers[i]);
    }
    println!("EIP = {:#010x}", emu.eip);
}

fn main() {
    // Read a filename from command argments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: x86_emu filename");
        process::exit(1);
    }
    let filename = &args[1];

    // Open a binary file that contains a x86 machine code
    let mut file = File::open(filename)
        .expect("file not found");

    // Create a emulator with EIP=0x0000_0000 and ESP=0x0000_7c00
    let mut emu = create_emu(0x0000_0000, 0x0000_7c00);
    
    // Load the binary file into the emulator's memory
    file.read_to_end(&mut emu.memory)
        .expect("something went wrong reading the file");

    // Initialize the x86 instructions table
    // The None value in the instructions table indicates that instruction is not implemented
    let mut instructions: [Option<fn(&mut Emulator)>; INSTRUCTIONS_COUNT]
        = [None; INSTRUCTIONS_COUNT];
    init_instructions(&mut instructions);

    // Emulation loop
    loop {
        // Read a instruction code
        let code: u8 = get_code8(&emu, 0);
        println!("EIP = {:#010x}, Code = {:#04x}", emu.eip, code);

        match instructions[code as usize] {
            // Execute the instruction
            Some(inst) => inst(&mut emu),
            // Stop the program if the instructin is not implemented
            None => {
                println!("\nNot Implemented: {:#04x}\n", code);
                break;
            }
        }

        // Stop the program when EIP=0
        if emu.eip == 0x0000_0000 {
            println!("\nend of program.\n");
            break;
        }
    }

    // Stop the program and dump registers
    dump_registers(&emu);
}
