mod cpu;
mod instruction;

use self::cpu::CPU;

pub struct Z80 {
    cpu: CPU,
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            cpu: CPU::new(),
        }
    }

    pub fn hard_reset(&mut self) {

    }

    pub fn run(&mut self) {
        let instr: u32 = 0xCB_00_06;

        self.cpu.do_instruction(instr);
    }
}