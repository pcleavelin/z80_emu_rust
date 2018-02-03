#[macro_use]
mod logging;

mod cpu;
mod cpu_status;
mod mmu;
mod instruction;

use minifb::{WindowOptions, Window, Key, Scale};

use self::cpu::Cpu;
use self::mmu::Mmu;

pub const VID_WIDTH: usize = 52 * 8;
pub const VID_HEIGHT: usize = 24 * 10;

pub const MAX_CHAR_ROM_SIZE: usize = 0x800;

pub struct Z80 {
    cpu: Cpu,
    mmu: Mmu,

    char_rom: [u8; MAX_CHAR_ROM_SIZE],

    window: Window,
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            cpu: Cpu::new(),
            mmu: Mmu::new(),

            char_rom: [0u8; MAX_CHAR_ROM_SIZE],

            window: Window::new("Z80 | Osborne 1 Emulator", VID_WIDTH, VID_HEIGHT, WindowOptions {
                borderless: false,
                title: true,
                resize: false,
                scale: Scale::X2,
            }).expect("Failed to create window!")
        }
    }

    pub fn poke_char_rom(&mut self, data: &[u8]) -> bool{
        if data.len() > MAX_CHAR_ROM_SIZE {
            return false;
        }

        for i in 0..data.len() {
            self.char_rom[i] = data[i];
        }

        true
    }

    pub fn poke_com_file(&mut self, data: &[u8]) -> bool {
        if data.len() > mmu::MAX_MMU_RAM {
            return false;
        }

        for i in 0..data.len() {
            self.mmu.write_mem(i as u16, data[i]);
        }

        return true;
    }

    pub fn hard_reset(&mut self) {
        self.cpu.set_pc(0);
    }

    pub fn run(&mut self) {
        let mut running = true;
        let mut cycles: u64 = 0;

        while running && self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let instr: u32 = (self.mmu.read_mem(self.cpu.reg_pc()) as u32) << 24
                                | (self.mmu.read_mem(self.cpu.reg_pc()+1) as u32) << 16
                                | (self.mmu.read_mem(self.cpu.reg_pc()+2) as u32) << 8
                                | (self.mmu.read_mem(self.cpu.reg_pc()+3) as u32);

            log!(Log::Debug, "0x{:08X}", instr);

            running = self.cpu.do_instruction(instr, &mut self.mmu);

            log!(Log::Debug, "\n");

            let mut vram = [0u32; VID_WIDTH*VID_HEIGHT];

            for i in 0..52 {
                for j in 0..24 {
                    for y in 0..10 {
                        let character = self.mmu.read_mem(0xF000 + i + y*128) as usize;
                        let row = self.char_rom[character + (y*128) as usize];

                        for x in 0..8 {
                            let color = (((row >> (7-x)) & 1) * 0xFF) as u32;

                            vram[((i*8)+x) as usize + ((j*10)+y) as usize *VID_WIDTH] = color << 16 | color << 8 | color;
                        }
                    }
                }
            }

            if cycles % 1024 == 0 {
                let _ = self.window.update_with_buffer(&vram);
                cycles = 0;
            }

            cycles += 1;
        }
    }
}