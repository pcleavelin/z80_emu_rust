use super::mmu::Mmu;
use super::instruction::Opcode;
use super::cpu_status::CPUStatus;

pub struct Cpu {
    reg_a: u8,
    reg_f: CPUStatus,

    reg_b: u8,
    reg_c: u8,

    reg_d: u8,
    reg_e: u8,

    reg_h: u8,
    reg_l: u8,

    reg_a_prime: u8,
    reg_f_prime: CPUStatus,

    reg_b_prime: u8,
    reg_c_prime: u8,

    reg_d_prime: u8,
    reg_e_prime: u8,

    reg_h_prime: u8,
    reg_l_prime: u8,


    reg_pc: u16,
    reg_sp: u16,
    reg_ix: u16,
    reg_iy: u16,

    reg_i: u8,
    reg_r: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_a: 0u8,
            reg_f: CPUStatus::from(0),

            reg_b: 0u8,
            reg_c: 0u8,

            reg_d: 0u8,
            reg_e: 0u8,

            reg_h: 0u8,
            reg_l: 0u8,


            reg_a_prime: 0u8,
            reg_f_prime: CPUStatus::from(0),

            reg_b_prime: 0u8,
            reg_c_prime: 0u8,

            reg_d_prime: 0u8,
            reg_e_prime: 0u8,

            reg_h_prime: 0u8,
            reg_l_prime: 0u8,


            reg_pc: 0u16,
            reg_sp: 0u16,
            reg_ix: 0u16,
            reg_iy: 0u16,

            reg_i: 0u8,
            reg_r: 0u8,
        }
    }

    pub fn set_pc(&mut self, val: u16) {
        self.reg_pc = val;
    }

    pub fn reg_pc(&self) -> u16 {
        self.reg_pc
    }

    pub fn push_stack(&mut self, mmu: &mut Mmu, val: u8) {
        self.reg_sp = self.reg_sp.wrapping_sub(1);
        mmu.write_mem(self.reg_sp, val);
    }

    pub fn pop_stack(&mut self, mmu: &Mmu) -> u8 {
        let val = mmu.read_mem(self.reg_sp);
        self.reg_sp = self.reg_sp.wrapping_add(1);
        
        val
    }

    pub fn call(&mut self, mmu: &mut Mmu, addr: u16) {
        let hi = ((self.reg_pc&0xFF00) >> 8) as u8;
        let lo = (self.reg_pc&0xFF) as u8;

        self.push_stack(mmu, hi);
        self.push_stack(mmu, lo);
        
        self.reg_pc = addr;
    }

    pub fn do_instruction(&mut self, instr: u32, mmu: &mut Mmu) -> bool {

        println!("pc: 0x{:04X}", self.reg_pc);

        // Mask out only the prefix and opcode
        let (op, delta_ip) = Opcode::from_halfword((instr>>16) as u16);

        // Offset PC depending on how big the instruction is
        self.reg_pc = self.reg_pc.wrapping_add(delta_ip as u16);

        // Get the immediate value
        let (imm_lo,imm_hi) = match delta_ip {
            1 => {
                (((instr&0x00_FF_00_00) >> 16) as u8,
                 ((instr&0x00_00_FF_00) >> 8) as u8)
            }
            2 => {
                (((instr&0x00_00_FF_00) >> 8) as u8,
                (instr&0x00_00_00_FF) as u8)
            }
            _ => {
                (0,0)
            }
        };

        match op {
            
            Opcode::CallIfCarry => {
                if self.reg_f.carry() {
                    self.call(mmu, (imm_hi as u16) << 8 | (imm_lo as u16));
                }
            }

            Opcode::ExAfAfPrime => {
                let a_prime = self.reg_a_prime;
                let f_prime = self.reg_f_prime;
                
                self.reg_a_prime = self.reg_a;
                self.reg_f_prime = self.reg_f;

                self.reg_a = a_prime;
                self.reg_f = f_prime;
            }

            Opcode::JmpRelIfNz => {
                if self.reg_f.zero() == false {
                    self.reg_pc = self.reg_pc.wrapping_add((imm_lo as i8) as u16);
                }
            }

            Opcode::LdBE => {
                self.reg_b = self.reg_e;
            }

            Opcode::Nop => {}

            Opcode::Invalid => {
                println!("Invalid opcode...crashing");
                return false;
            }

            _ => {
                println!("Unimplemented opcode...crashing");
                return false;
            }
        }

        true
    }
}