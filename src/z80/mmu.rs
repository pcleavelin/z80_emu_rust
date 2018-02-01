pub const MAX_MMU_RAM: usize = 0x10000;

pub struct Mmu {
    ram: [u8; MAX_MMU_RAM],
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            ram: [0; MAX_MMU_RAM]
        }
    }

    pub fn write_mem(&mut self, addr: u16, val: u8) {
        if addr as usize >= MAX_MMU_RAM {
            println!("Writing past RAM!");
            return;
        }

        self.ram[addr as usize] = val;
    }

    pub fn read_mem(&self, addr: u16) -> u8{
        if addr as usize >= MAX_MMU_RAM {
            println!("Reading past RAM!");
            return 0;
        }

        self.ram[addr as usize]
    }
}