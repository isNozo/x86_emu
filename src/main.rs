#![allow(dead_code)]

mod emulator;
mod function;
mod instruction;

use std::env;
use std::process;
use std::fs::File;
use emulator::*;
use function::*;
use instruction::*;

const MEMORY_SIZE: usize = 1024 * 1024;

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

    // Create a emulator with EIP=0x0000_7c00 and ESP=0x0000_7c00
    let mut emu = create_emu(MEMORY_SIZE, 0x0000_7c00, 0x0000_7c00);
    
    // Load the binary file into the emulator's memory
    read_to_memory(&mut file, &mut emu)
        .expect("something went wrong reading the file");

    // Initialize the x86 instructions table
    // The None value in the instructions table indicates that instruction is not implemented
    let mut instructions: Instructoins = [None; INSTRUCTIONS_COUNT];
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
