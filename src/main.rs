#![allow(dead_code)]

use std::env;
use std::process;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use Register::*;

// General-purpose Registers Set
// This enum is used as index to access the registers[] field of Emulator
enum Register { EAX=0, ECX, EDX, EBX, ESP, EBP, ESI, EDI }
const REGISTERS_COUNT: usize = 8;
const REGISTERS_NAME: [&str; REGISTERS_COUNT] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

const MEMORY_SIZE: usize = 1024 * 1024;
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
fn create_emu(mem_size: usize, eip: u32, esp: u32) -> Emulator {
    let mut emu = Emulator {
        // Clear all resisters by 0
        registers: [0; REGISTERS_COUNT],
        // Clear eflags by 0
        eflags: 0,
        // Init EIP register
        eip: eip,
        // Init memory
        memory: vec![0; mem_size]
    };

    // Init ESP register
    emu.registers[ESP as usize] = esp;

    emu
}

/* 
 * Read a byte code from a emulator's memory
 */
fn get_code8(emu: &Emulator, offset: usize) -> u8 {
    emu.memory[emu.eip as usize + offset]
}

fn get_sign_code8(emu: &Emulator, offset: usize) -> i8 {
    get_code8(emu, offset) as i8
}

fn get_code32(emu: &Emulator, offset: usize) -> u32 {
    let mut ret: u32 = 0x0000_00000;
    
    // Get a 32bit data as little endian
    for i in 0..4 {
        ret |= (get_code8(emu, offset + i) as u32) << (i * 8);
    }
    ret
}

fn get_sign_code32(emu: &Emulator, offset: usize) -> i32 {
    get_code32(emu, offset) as i32
}

/* 
 * Define x86 instructions
 */
fn mov_r32_imm32(emu: &mut Emulator) {
    // Get a target register from opecode
    let reg = get_code8(emu, 0) - 0xB8;
    // Get 32bit immediate data from operand
    let imm = get_code32(emu, 1);
    // Set immediate data to the target register
    emu.registers[reg as usize] = imm;
    // Count up the EIP register
    emu.eip += 5;
}

fn short_jump(emu: &mut Emulator) {
    // Get a 8bit jump diff
    let diff = get_sign_code8(emu, 1);
    // Add the diff to the EIP register
    emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}

fn near_jump(emu: &mut Emulator) {
    let diff = get_sign_code32(emu, 1);
    emu.eip = emu.eip.wrapping_add((diff + 5) as u32);
}

// The Instructions type is a function pointer array
const INSTRUCTIONS_COUNT: usize = 256;
type Instructoins = [Option<fn(&mut Emulator)>; INSTRUCTIONS_COUNT];

// Initialize a instructions table
fn init_instructions(instructions: &mut Instructoins) {
    for i in 0..8 {
        instructions[0xB8 + i] = Some(mov_r32_imm32);
    }

    instructions[0xE9] = Some(near_jump);
    instructions[0xEB] = Some(short_jump);
}

// Dump general-purpose registers and EIP values
fn dump_registers(emu: &Emulator) {
    for i in 0..REGISTERS_COUNT {
        println!("{} = {:#010x}", REGISTERS_NAME[i], emu.registers[i]);
    }
    println!("EIP = {:#010x}", emu.eip);
}

const BIOS_OFFSET: usize = 0x7c00;

// Read a file to the memory of the emulator
fn read_to_memory(file: &mut File, emu: &mut Emulator) -> Result<usize, io::Error> {
    let mut cnt = 0;

    for byte in file.bytes() {
        emu.memory[BIOS_OFFSET + cnt] = byte?;
        cnt += 1;
    }

    Ok(cnt)
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
