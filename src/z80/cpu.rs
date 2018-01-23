use super::instruction::Opcode;

pub struct CPU {
    reg_a: u8,
    reg_f: u8,

    reg_b: u8,
    reg_c: u8,

    reg_d: u8,
    reg_e: u8,

    reg_h: u8,
    reg_l: u8,

    reg_a_prime: u8,
    reg_f_prime: u8,

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

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg_a: 0u8,
            reg_f: 0u8,

            reg_b: 0u8,
            reg_c: 0u8,

            reg_d: 0u8,
            reg_e: 0u8,

            reg_h: 0u8,
            reg_l: 0u8,


            reg_a_prime: 0u8,
            reg_f_prime: 0u8,

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

    pub fn do_instruction(&mut self, instr: u32) -> bool {
        let op = Opcode::from_halfword(((instr&0xFF_FF_00)>>8) as u16);

        false
    }
}