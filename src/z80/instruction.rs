use std::fmt;
use std::fmt::{Display, Formatter};

pub enum Opcode {
    AddHlBc,

    AndAN,

    OrAC,

    XorAIndHl,

    CpC,

    CallIfCarry,
    Call,
    Ret,

    Di,

    DecA,
    DecC,

    ExAfAfPrime,

    IncL,

    Jp,

    JrIfNc,
    JrIfNz,
    JrIfZ,

    LdAC,
    LdAN,
    LdAIndNN,
    LdBA,
    LdBB,
    LdBE,
    LdCA,
    LdCC,
    LdIndNNA,
    LdHlNN,
    LdSpNN,

    Rrca,

    Nop,
    Invalid,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Opcode::AddHlBc => write!(f, "AddHlBc (ADD HL, BC)"),

            &Opcode::AndAN => write!(f, "AndAN (AND A, n)"),

            &Opcode::OrAC => write!(f, "OrAC (OR A, C)"),

            &Opcode::XorAIndHl => write!(f, "XorAIndHl (XOR A, (HL))"),

            &Opcode::CpC => write!(f, "CpC (CP A, C)"),

            &Opcode::CallIfCarry => write!(f, "CallIfCarry (CALL C, nn)"),
            &Opcode::Call => write!(f, "Call (CALL nn)"),
            &Opcode::Ret => write!(f, "Ret (RET)"),

            &Opcode::Di => write!(f, "Di (DI)"),

            &Opcode::DecA => write!(f, "DecA (DEC A)"),
            &Opcode::DecC => write!(f, "DecC (DEC C)"),

            &Opcode::ExAfAfPrime => write!(f, "ExAfAfPrime (EX AF, AF')"),

            &Opcode::IncL => write!(f, "INC L"),

            &Opcode::Jp => write!(f, "Jp (JP nn)"),

            &Opcode::JrIfNc => write!(f, "JrIfNc (JR NC, d)"),
            &Opcode::JrIfNz => write!(f, "JrIfNz (JR NZ, d)"),
            &Opcode::JrIfZ => write!(f, "JrIfZ (JR Z, d)"),

            &Opcode::LdAC => write!(f, "LdAC (LD A, C)"),
            &Opcode::LdAN => write!(f, "LdAN (LD A, n)"),
            &Opcode::LdAIndNN => write!(f, "LdAIndNN (LD A, (nn))"),
            &Opcode::LdBA => write!(f, "LdBA (LD B, A)"),
            &Opcode::LdBB => write!(f, "LdBB (LD B, B)"),
            &Opcode::LdBE => write!(f, "LdBE (LD B, E)"),
            &Opcode::LdCA => write!(f, "LdCA (LD C, A)"),
            &Opcode::LdCC => write!(f, "LdCC (LD C, C)"),
            &Opcode::LdIndNNA => write!(f, "LdIndNNA (LD (nn), A)"),
            &Opcode::LdHlNN => write!(f, "LdHlNN (LD HL, nn)"),
            &Opcode::LdSpNN => write!(f, "LdSpNN (LD SP, nn)"),

            &Opcode::Rrca => write!(f, "Rrca (RRCA)"),

            &Opcode::Nop => write!(f, "Nop (NOP)"),
            &Opcode::Invalid => write!(f, "Invalid"),
        }
    }
}

impl Opcode {
    pub fn from_halfword(halfword: u16) -> (Opcode, usize, bool) {
        log!(Log::Debug, "from_halfword 0x{:04X}", halfword);

        match (halfword&0xFF00) >> 8 {
            0xCB => {
                let (op, ip) = Opcode::from_cb_prefix((halfword&0xFF) as u8);
                (op, ip, true)
            }

            0xED => {
                let (op, ip) = Opcode::from_ed_prefix((halfword&0xFF) as u8);
                (op, ip, true)
            }

            0xDD => {
                let (op, ip) = Opcode::from_dd_prefix((halfword&0xFF) as u8);
                (op, ip, true)
            }
            
            0xFD => {
                let (op, ip) = Opcode::from_fd_prefix((halfword&0xFF) as u8);
                (op, ip, true)
            }

            // Un-Prefixed Opcodes
            _ => {
                let (op, ip) = Opcode::from_unprefixed(((halfword&0xFF00) >> 8) as u8);
                (op, ip, false)
            }
        }
    }

    pub fn from_cb_prefix(byte: u8) -> (Opcode, usize) {
        log!(Log::Debug, "from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                log!(Log::Debug, "unrecognized cb-prefixed opcode 0x{:02X}", byte);
                (Opcode::Invalid, 1)
            }
        }
    }

    pub fn from_ed_prefix(byte: u8) -> (Opcode, usize) {
        log!(Log::Debug, "from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                log!(Log::Debug, "unrecognized ed-prefixed opcode 0x{:02X}", byte);
                (Opcode::Invalid, 1)
            }
        }
    }

    pub fn from_dd_prefix(byte: u8) -> (Opcode, usize) {
        log!(Log::Debug, "from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                log!(Log::Debug, "unrecognized dd-prefixed opcode 0x{:02X}", byte);
                (Opcode::Invalid, 1)
            }
        }
    }

    pub fn from_fd_prefix(byte: u8) -> (Opcode, usize) {
        log!(Log::Debug, "from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                log!(Log::Debug, "unrecognized fd-prefixed opcode 0x{:02X}", byte);
                (Opcode::Invalid, 1)
            }
        }
    }

    pub fn from_unprefixed(byte: u8) -> (Opcode, usize) {
        log!(Log::Debug, "from_unprefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        // See http://z80.info/decoding.htm
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        let p = y >> 1;
        let q = y % 2;

        log!(Log::Debug, "x: {}, z: {}, y: {}\nq: {}, p: {}", x,z,y,q,p);
        
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
                                log!(Log::Instr, "JR NZ, d");
                                return (Opcode::JrIfNz, 2);
                            }
                            5 => {
                                log!(Log::Instr, "JR Z, d");
                                return (Opcode::JrIfZ, 2);
                            }
                            6 => {
                                log!(Log::Instr, "JR NC, d");
                                return (Opcode::JrIfNc, 2);
                            }
                            _ => {}
                        }
                    }
                    1 => {
                        match y {
                            4 => {
                                log!(Log::Instr, "LD HL, nn");
                                return (Opcode::LdHlNN, 3);
                            }
                            5 => {
                                log!(Log::Instr, "ADD HL, BC");
                                return (Opcode::AddHlBc, 1);
                            }
                            6 => {
                                log!(Log::Instr, "LD SP, nn");
                                return (Opcode::LdSpNN, 3);
                            }
                            _ => {}
                        }
                    }
                    2 => {
                        match y {
                            6 => {
                                log!(Log::Instr, "LD (nn), A");
                                return (Opcode::LdIndNNA, 3);
                            }
                            7 => {
                                log!(Log::Instr, "LD A, (nn)");
                                return (Opcode::LdAIndNN, 3);
                            }
                            _ => {}
                        }
                    }
                    4 => {
                        match y {
                            5 => {
                                log!(Log::Instr, "INC L");
                                return (Opcode::IncL, 1);
                            }
                            _ => {}
                        }
                    }
                    5 => {
                        match y {
                            1 => {
                                log!(Log::Instr, "DEC C");
                                return (Opcode::DecC, 1);
                            }
                            7 => {
                                log!(Log::Instr, "DEC A");
                                return (Opcode::DecA, 1);
                            }
                            _ => {}
                        }
                    }
                    6 => {
                        match y {
                            7 => {
                                log!(Log::Instr, "LD A, n");
                                return (Opcode::LdAN, 2);
                            }
                            _ => {}
                        }
                    }
                    7 => {
                        match y {
                            1 => {
                                log!(Log::Instr, "RRCA");
                                return (Opcode::Rrca, 1);
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
                    0 => {
                        match y {
                            0 => {
                                log!(Log::Instr, "LD B, B");
                                return (Opcode::LdBB, 1);
                            }
                            _ => {}
                        }
                    }
                    1 => {
                        match y {
                            1 => {
                                log!(Log::Instr, "LD C, C");
                                return (Opcode::LdCC, 1);
                            }
                            7 => {
                                log!(Log::Instr, "LD A, C");
                                return (Opcode::LdAC, 1);
                            }
                            _ => {}
                        }
                    }
                    3 => {
                        match y {
                            0 => {
                                log!(Log::Instr, "LD B, E");
                                return (Opcode::LdBE, 1);
                            }
                            _ => {}
                        }
                    }
                    7 => {
                        match y {
                            0 => {
                                log!(Log::Instr, "LA B, A");
                                return (Opcode::LdBA, 1);
                            }
                            1 => {
                                log!(Log::Instr, "LD C, A");
                                return (Opcode::LdCA, 1);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            
            2 => {
                match z {
                    1 => {
                        match y {
                            6 => {
                                log!(Log::Instr, "OR A, C");
                                return (Opcode::OrAC, 1);
                            }
                            7 => {
                                log!(Log::Instr, "CP C");
                                return (Opcode::CpC, 1);
                            }

                            _ => {}
                        }
                    }
                    6 => {
                        match y {
                            5 => {
                                log!(Log::Instr, "XOR A, (HL)");
                                return (Opcode::XorAIndHl, 1);
                            }

                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            3 => {
                match z {
                    1 => {
                        match y {
                            1 => {
                                log!(Log::Instr, "RET");
                                return (Opcode::Ret, 1);
                            }

                            _ => {}
                        }
                    }
                    3 => {
                        match y {
                            0 => {
                                log!(Log::Instr, "JP nn");
                                return (Opcode::Jp, 3);
                            }
                            6 => {
                                log!(Log::Instr, "DI");
                                return (Opcode::Di, 1);
                            }

                            _ => {}
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

                    5 => {
                        match y {
                            1 => {
                                log!(Log::Instr, "CALL nn");
                                return (Opcode::Call, 3);
                            }
                            _ => {}
                        }
                    }

                    6 => {
                        log!(Log::Instr, "AND A, n");
                        return (Opcode::AndAN, 2);
                    }

                    _ => {}
                }
            }

            _ => {}
        }

        log!(Log::Debug, "unrecognized unprefixed opcode 0x{:02X}", byte);
        (Opcode::Invalid, 1)
    }
}