use crate::emulator::*;
use crate::function::*;
use crate::modrm::*;

// The Instructions type is a function pointer array
pub const INSTRUCTIONS_COUNT: usize = 256;
pub type Instructoins = [Option<fn(&mut Emulator)>; INSTRUCTIONS_COUNT];

// Initialize a instructions table
pub fn init_instructions(instructions: &mut Instructoins) {
    for i in 0..8 {
        instructions[0xB8 + i] = Some(mov_r32_imm32);
    }

    instructions[0xE9] = Some(near_jump);
    instructions[0xEB] = Some(short_jump);
}

pub fn mov_r32_imm32(emu: &mut Emulator) {
    // Get a target register from opecode
    let reg = get_code8(emu, 0) - 0xB8;
    // Get 32bit immediate data from operand
    let imm = get_code32(emu, 1);
    // Set immediate data to the target register
    emu.registers[reg as usize] = imm;
    // Count up the EIP register
    emu.eip += 5;
}

pub fn mov_rm32_imm32(emu: &mut Emulator) {
    emu.eip += 1;
    let modrm = parse_modrm(emu);
    let value = get_code32(emu, 0);
    emu.eip += 4;
    set_rm32(emu, &modrm, value);
}

pub fn short_jump(emu: &mut Emulator) {
    // Get a 8bit jump diff
    let diff = get_sign_code8(emu, 1);
    // Add the diff to the EIP register
    emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}

pub fn near_jump(emu: &mut Emulator) {
    let diff = get_sign_code32(emu, 1);
    emu.eip = emu.eip.wrapping_add((diff + 5) as u32);
}
