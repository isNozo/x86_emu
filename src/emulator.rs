use std::fs::File;
use std::io;
use std::io::prelude::*;

// General-purpose Registers Set
// This enum is used as index to access the registers[] field of Emulator
pub enum Register { EAX=0, ECX, EDX, EBX, ESP, EBP, ESI, EDI }
pub const REGISTERS_COUNT: usize = 8;
pub const REGISTERS_NAME: [&str; REGISTERS_COUNT] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];
pub const BIOS_OFFSET: usize = 0x7c00;

pub struct Emulator {
    // General-purpose Registers
    pub registers: [u32; REGISTERS_COUNT],
    // EFLAGS Register
    pub eflags: u32,
    // Instruction Pointer
    pub eip: u32,
    // Memory
    pub memory: Vec<u8>
}

// Create a new instance of the emulator with EIP and ESP
pub fn create_emu(mem_size: usize, eip: u32, esp: u32) -> Emulator {
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
    emu.registers[Register::ESP as usize] = esp;

    emu
}

// Dump general-purpose registers and EIP values
pub fn dump_registers(emu: &Emulator) {
    for i in 0..REGISTERS_COUNT {
        println!("{} = {:#010x}", REGISTERS_NAME[i], emu.registers[i]);
    }
    println!("EIP = {:#010x}", emu.eip);
}

// Read a file to the memory of the emulator
pub fn read_to_memory(file: &mut File, emu: &mut Emulator) -> Result<usize, io::Error> {
    let mut cnt = 0;

    for byte in file.bytes() {
        emu.memory[BIOS_OFFSET + cnt] = byte?;
        cnt += 1;
    }

    Ok(cnt)
}
