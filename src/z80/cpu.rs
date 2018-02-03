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

    interrupt_enable: bool,
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

            interrupt_enable: true,
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

    pub fn ret(&mut self, mmu: &mut Mmu) {
        let lo = self.pop_stack(mmu) as u16;
        let hi = self.pop_stack(mmu) as u16;

        self.reg_pc = (hi << 8) | lo;
    }

    pub fn do_instruction(&mut self, instr: u32, mmu: &mut Mmu) -> bool {

        println!("pc: 0x{:04X}", self.reg_pc);

        // Mask out only the prefix and opcode
        let (op, delta_ip, is_prefixed) = Opcode::from_halfword((instr>>16) as u16);

        if delta_ip > 4 {
            println!("delta_ip > 4 for opcode {}", op);
            return false;
        }

        // Offset PC depending on how big the instruction is
        self.reg_pc = self.reg_pc.wrapping_add(delta_ip as u16);

        // Get the immediate value
        let (imm_lo,imm_hi) = match is_prefixed {
            false => {
                (((instr&0x00_FF_00_00) >> 16) as u8,
                 ((instr&0x00_00_FF_00) >> 8) as u8)
            }

            true => {
                (((instr&0x00_00_FF_00) >> 8) as u8,
                (instr&0x00_00_00_FF) as u8)
            }
        };
        println!("imm_lo: 0x{:02X}\nimm_hi: 0x{:02X}\n{}", imm_lo, imm_hi, op);

        match op {
            Opcode::AndAN => {
                self.reg_a = self.reg_a & imm_lo;

                self.reg_f.set_sign(self.reg_a&0x80 > 0);
                self.reg_f.set_zero(self.reg_a == 0);
                self.reg_f.set_half_carry(true);
                self.reg_f.set_overflow(false);
                self.reg_f.set_add_sub(false);
                self.reg_f.set_carry(false);
            }

            Opcode::OrAC => {
                self.reg_a = self.reg_a | self.reg_c;

                self.reg_f.set_sign(self.reg_a&0x80 > 0);
                self.reg_f.set_zero(self.reg_a == 0);
                self.reg_f.set_half_carry(false);
                self.reg_f.set_overflow(false); //TODO: wat is parity
                self.reg_f.set_add_sub(false);
                self.reg_f.set_carry(false);
            }

            Opcode::XorAIndHl => {
                self.reg_a = self.reg_a ^ mmu.read_mem(((self.reg_h as u16) << 8) | (self.reg_l as u16));

                self.reg_f.set_sign(self.reg_a&0x80 > 0);
                self.reg_f.set_zero(self.reg_a == 0);
                self.reg_f.set_half_carry(false);
                self.reg_f.set_overflow(false); //TODO: wat is parity
                self.reg_f.set_add_sub(false);
                self.reg_f.set_carry(false);
            }

            Opcode::CpC => {
                let result = self.reg_a.wrapping_sub(self.reg_c);

                self.reg_f.set_sign(result&0x80 > 0);
                self.reg_f.set_zero(result == 0);
                self.reg_f.set_half_carry(result&0x20 > 0);
                self.reg_f.set_overflow(false); //TODO: wat is parity
                self.reg_f.set_add_sub(true);
                self.reg_f.set_carry(self.reg_a < self.reg_c); // ?? maybe
            }

            Opcode::CallIfCarry => {
                if self.reg_f.carry() {
                    self.call(mmu, (imm_hi as u16) << 8 | (imm_lo as u16));
                }
            }
            Opcode::Call => {
                self.call(mmu, (imm_hi as u16) << 8 | (imm_lo as u16));
            }
            Opcode::Ret => {
                self.ret(mmu);
            }

            Opcode::Di => {
                self.interrupt_enable = false;
            }

            Opcode::DecA => {
                self.reg_f.set_overflow(self.reg_a == 0x80);
                self.reg_a = self.reg_a.wrapping_sub(1);

                self.reg_f.set_sign(self.reg_a&0x80 > 0);
                self.reg_f.set_zero(self.reg_a == 0);
                self.reg_f.set_half_carry(self.reg_a&0x20 > 0);
                self.reg_f.set_add_sub(true);
            }

            Opcode::ExAfAfPrime => {
                let a_prime = self.reg_a_prime;
                let f_prime = self.reg_f_prime;
                
                self.reg_a_prime = self.reg_a;
                self.reg_f_prime = self.reg_f;

                self.reg_a = a_prime;
                self.reg_f = f_prime;
            }

            Opcode::IncL => {
                self.reg_f.set_overflow(self.reg_l == 0x7F);
                self.reg_l = self.reg_l.wrapping_add(1);

                self.reg_f.set_sign(self.reg_l&0x80 > 0);
                self.reg_f.set_zero(self.reg_l == 0);
                self.reg_f.set_half_carry(self.reg_l&0x10 > 0);
                self.reg_f.set_add_sub(false);
            }

            Opcode::Jp => {
                self.reg_pc = (imm_hi as u16) << 8 | (imm_lo as u16);
            }

            Opcode::JrIfNc => {
                if self.reg_f.carry() == false {
                    self.reg_pc = self.reg_pc.wrapping_add((imm_lo as i8) as u16);
                }
            }
            Opcode::JrIfNz => {
                if self.reg_f.zero() == false {
                    self.reg_pc = self.reg_pc.wrapping_add((imm_lo as i8) as u16);
                }
            }
            Opcode::JrIfZ => {
                if self.reg_f.zero() == true {
                    self.reg_pc = self.reg_pc.wrapping_add((imm_lo as i8) as u16);
                }
            }

            Opcode::LdAC => {
                self.reg_a = self.reg_c;
            }
            Opcode::LdAN => {
                self.reg_a = imm_lo;
            }
            Opcode::LdAIndNN => {
                self.reg_a = mmu.read_mem((imm_hi as u16) << 8 | (imm_lo as u16));
            }
            Opcode::LdBA => {
                self.reg_b = self.reg_a;
            }
            Opcode::LdBE => {
                self.reg_b = self.reg_e;
            }
            Opcode::LdCA => {
                self.reg_c = self.reg_a;
            }
            Opcode::LdIndNNA => {
                mmu.write_mem((imm_hi as u16) << 8 | (imm_lo as u16), self.reg_a);
            }
            Opcode::LdHlNN => {
                self.reg_h = imm_hi;
                self.reg_l = imm_lo;
            }

            Opcode::Rrca => {
                self.reg_f.set_carry(self.reg_a > 0);
                self.reg_a = (self.reg_a >> 1) | ((self.reg_f.carry() as u8) << 7);

                self.reg_f.set_half_carry(false);
                self.reg_f.set_add_sub(false);
            }

            Opcode::Nop => {}

            Opcode::Invalid => {
                println!("Invalid opcode...crashing");
                return false;
            }

            _ => {
                println!("Unimplemented opcode {}...crashing", op);
                return false;
            }
        }

        println!("A: 0x{:02X}", self.reg_a);
        println!("B: 0x{:02X}", self.reg_b);
        println!("C: 0x{:02X}", self.reg_c);
        println!("D: 0x{:02X}", self.reg_d);
        println!("E: 0x{:02X}", self.reg_e);
        println!("H: 0x{:02X}", self.reg_h);
        println!("L: 0x{:02X}", self.reg_l);

        true
    }
}