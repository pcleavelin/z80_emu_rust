
pub enum Opcode {
    AddImmAcc,

    CallIfCarry,

    ExAfAfPrime,

    JmpRelIfNz,

    LdBE,

    Nop,

    Invalid,
}

impl Opcode {
    pub fn from_halfword(halfword: u16) -> (Opcode, usize) {
        println!("from_halfword 0x{:04X}", halfword);

        match (halfword&0xFF00) >> 8 {
            0xCB => {
                Opcode::from_cb_prefix((halfword&0xFF) as u8)
            }

            0xED => {
                Opcode::from_ed_prefix((halfword&0xFF) as u8)
            }

            0xDD => {
                Opcode::from_dd_prefix((halfword&0xFF) as u8)
            }
            
            0xFD => {
                Opcode::from_fd_prefix((halfword&0xFF) as u8)
            }

            // Un-Prefixed Opcodes
            _ => {
                Opcode::from_unprefixed(((halfword&0xFF00) >> 8) as u8)
            }
        }
    }

    pub fn from_cb_prefix(byte: u8) -> (Opcode, usize) {
        println!("from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                println!("unrecognized cb-prefixed opcode 0x{:02X}", byte);
                (Opcode::Invalid, 1)
            }
        }
    }

    pub fn from_ed_prefix(byte: u8) -> (Opcode, usize) {
        println!("from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                println!("unrecognized ed-prefixed opcode 0x{:02X}", byte);
                (Opcode::Invalid, 1)
            }
        }
    }

    pub fn from_dd_prefix(byte: u8) -> (Opcode, usize) {
        println!("from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                println!("unrecognized dd-prefixed opcode 0x{:02X}", byte);
                (Opcode::Invalid, 1)
            }
        }
    }

    pub fn from_fd_prefix(byte: u8) -> (Opcode, usize) {
        println!("from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                println!("unrecognized fd-prefixed opcode 0x{:02X}", byte);
                (Opcode::Invalid, 1)
            }
        }
    }

    pub fn from_unprefixed(byte: u8) -> (Opcode, usize) {
        println!("from_unprefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        let p = y >> 1;
        let q = y % 2;

        println!("x: {}, y: {}, z: {}\nq: {}, p: {}", x,y,z,q,p);
        
        match x {
            0 => {
                match z {
                    0 => {
                        match y {
                            0 => {
                                log!(Log::Instr, "NOP");
                                return (Opcode::Nop, 1);
                            }

                            1 => {
                                log!(Log::Instr, "EX AF, AF'");
                                return (Opcode::ExAfAfPrime, 1);
                            }

                            4 => {
                                log!(Log::Instr, "JR NZ, n");
                                return (Opcode::JmpRelIfNz, 2);
                            }
                            _ => {}
                        }
                    }
                    _ => {

                    }
                }
            }

            1 => {
                match z {
                    0...5 => {
                        log!(Log::Instr, "LD B, E");
                        return (Opcode::LdBE, 1);
                    }
                    _ => {}
                }
            }

            3 => {
                match z {
                    3 => {
                        match y {
                            0 => {
                                log!(Log::Instr, "JP nn");
                                return (Opcode::Invalid, 2);
                            }

                            _ => {
                                println!("unrecognized z-part of opcode 0b{:03b}", z);
                            }
                        }
                    }

                    4 => {
                        match y {
                            3 => {
                                log!(Log::Instr, "CALL C, nn");
                                return (Opcode::CallIfCarry, 3);
                            }
                            _ => {}
                        }
                    }

                    6 => {
                        return (Opcode::AddImmAcc, 2);
                    }

                    _ => {
                        println!("unrecognized z-part of opcode 0b{:03b}", z);
                    }
                }
            }

            _ => {}
        }

        println!("unrecognized unprefixed opcode 0x{:02X}", byte);
        (Opcode::Invalid, 1)
    }
}