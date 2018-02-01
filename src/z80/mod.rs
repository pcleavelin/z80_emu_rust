#[macro_use]
mod logging;

mod cpu;
mod cpu_status;
mod mmu;
mod instruction;

use self::cpu::Cpu;
use self::mmu::Mmu;

pub struct Z80 {
    cpu: Cpu,
    mmu: Mmu,
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            cpu: Cpu::new(),
            mmu: Mmu::new(),
        }
    }

    pub fn poke_com_file(&mut self, data: &[u8]) -> bool {
        if data.len() >= mmu::MAX_MMU_RAM {
            return false;
        }

        for i in 0..data.len() {
            self.mmu.write_mem(i as u16, data[i]);
        }

        return true;
    }

    pub fn hard_reset(&mut self) {
        self.cpu.set_pc(0x10);
    }

    pub fn run(&mut self) {
        let mut running = true;
        while running {
            let instr: u32 = (self.mmu.read_mem(self.cpu.reg_pc()) as u32) << 24
                                | (self.mmu.read_mem(self.cpu.reg_pc()+1) as u32) << 16
                                | (self.mmu.read_mem(self.cpu.reg_pc()+2) as u32) << 8
                                | (self.mmu.read_mem(self.cpu.reg_pc()+3) as u32);

            println!("0x{:08X}", instr);

            running = self.cpu.do_instruction(instr, &mut self.mmu);

            println!("\n");
        }
    }
}