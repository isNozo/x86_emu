use crate::emulator::*;

/* 
 * Read a byte code from a emulator's memory
 */
pub fn get_code8(emu: &Emulator, offset: usize) -> u8 {
    emu.memory[emu.eip as usize + offset]
}

pub fn get_sign_code8(emu: &Emulator, offset: usize) -> i8 {
    get_code8(emu, offset) as i8
}

pub fn get_code32(emu: &Emulator, offset: usize) -> u32 {
    let mut ret: u32 = 0x0000_00000;
    
    // Get a 32bit data as little endian
    for i in 0..4 {
        ret |= (get_code8(emu, offset + i) as u32) << (i * 8);
    }
    ret
}

pub fn get_sign_code32(emu: &Emulator, offset: usize) -> i32 {
    get_code32(emu, offset) as i32
}
