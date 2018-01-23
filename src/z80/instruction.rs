pub enum Opcode {
    AddImmAcc,
}

impl Opcode {
    pub fn from_halfword(halfword: u16) -> Opcode {
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
                Opcode::from_unprefixed((halfword&0xFF) as u8)
            }
        }
    }

    pub fn from_cb_prefix(byte: u8) -> Opcode {
        println!("from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                panic!("unrecognized cb-prefixed opcode 0x{:02X}", byte);
            }
        }
    }

    pub fn from_ed_prefix(byte: u8) -> Opcode {
        println!("from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                panic!("unrecognized ed-prefixed opcode 0x{:02X}", byte);
            }
        }
    }

    pub fn from_dd_prefix(byte: u8) -> Opcode {
        println!("from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                panic!("unrecognized dd-prefixed opcode 0x{:02X}", byte);
            }
        }
    }

    pub fn from_fd_prefix(byte: u8) -> Opcode {
        println!("from_prefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;

        match x {
            _ => {
                panic!("unrecognized fd-prefixed opcode 0x{:02X}", byte);
            }
        }
    }

    pub fn from_unprefixed(byte: u8) -> Opcode {
        println!("from_unprefixed 0x{:02X}", byte);

        // Different parts of the byte change what opcode ends up being decoded
        let x = (byte&0xC0) >> 6;
        let y = (byte&0x38) >> 3;
        let z = byte&0x7;
        
        match x {

            3 => {
                match z {
                    6 => {
                        Opcode::AddImmAcc
                    }

                    _ => {
                        panic!("unrecognized z-part of opcode 0b{:03b}", z);
                    }
                }
            }

            _ => {
                panic!("unrecognized unprefixed opcode 0x{:02X}", byte);
            }
        }
    }
}