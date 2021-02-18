use crate::emulator::*;

pub struct ModRM {
}

pub fn parse_modrm(emu: &Emulator) -> ModRM {
    ModRM {}
}

pub fn set_rm32(emu: &mut Emulator, modrm: &ModRM, value: u32) {
}