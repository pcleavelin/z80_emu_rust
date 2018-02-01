use std::ops::{BitOr, BitAnd};

#[derive(Copy, Clone, Debug)]
pub struct CPUStatus {
    Carry: bool,
    AddSub: bool,
    Overflow: bool,
    HalfCarry: bool,
    Zero: bool,
    Sign: bool,

    bit_3: bool,
    bit_5: bool,
}

impl From<u8> for CPUStatus {
    fn from(val: u8) -> CPUStatus {
        CPUStatus {
            Carry: (val&0x1) > 0,
            AddSub: (val&0x2) > 0,
            Overflow: (val&0x4) > 0,
            bit_3: (val&0x8) > 0,
            bit_5: (val&0x20) > 0,
            Zero: (val&0x40) > 0,
            Sign: (val&0x80) > 0,

            HalfCarry: false,
        }
    }
}

impl BitOr<u8> for CPUStatus {
    type Output = CPUStatus;

    fn bitor(self, val: u8) -> CPUStatus {
        CPUStatus::from(self.to_u8() | val)
    }
}

impl BitAnd<u8> for CPUStatus {
    type Output = CPUStatus;

    fn bitand(self, val: u8) -> CPUStatus {
        CPUStatus::from(self.to_u8() & val)
    }
}

impl CPUStatus {
    pub fn new() -> CPUStatus {
        CPUStatus {
            Carry: false,
            AddSub: false,
            Overflow: false,
            HalfCarry: false,
            Zero: false,
            Sign: false,

            bit_3: false,
            bit_5: false,
        }
    }

    pub fn carry(&self) -> bool {
        self.Carry
    }
    pub fn add_sub(&self) -> bool {
        self.AddSub
    }
    pub fn overflow(&self) -> bool {
        self.Overflow
    }
    pub fn half_carry(&self) -> bool {
        self.HalfCarry
    }
    pub fn zero(&self) -> bool {
        self.Zero
    }
    pub fn sign(&self) -> bool {
        self.Sign
    }
    pub fn bit_3(&self) -> bool {
        self.bit_3
    }
    pub fn bit_5(&self) -> bool {
        self.bit_5
    }

    pub fn set_carry(&mut self, val: bool) {
        self.Carry = val;
    }
    pub fn set_add_sub(&mut self, val: bool) {
        self.AddSub = val;
    }
    pub fn set_overflow(&mut self, val: bool) {
        self.Overflow = val;
    }
    pub fn set_half_carry(&mut self, val: bool) {
        self.HalfCarry = val;
    }
    pub fn set_zero(&mut self, val: bool) {
        self.Zero = val;
    }
    pub fn set_sign(&mut self, val: bool) {
        self.Sign = val;
    }
    pub fn set_bit_3(&mut self, val: bool) {
        self.bit_3 = val;
    }
    pub fn set_bit_5(&mut self, val: bool) {
        self.bit_5 = val;
    }

    pub fn to_u8(&self) -> u8 {
        let mut val = self.Carry as u8;
        val |= (self.AddSub as u8) << 1;
        val |= (self.Overflow as u8) << 2;
        val |= (self.bit_3 as u8) << 3;
        val |= (self.AddSub as u8) << 4;
        val |= (self.bit_5 as u8) << 5;
        val |= (self.Zero as u8) << 6;
        val |= (self.Sign as u8) << 7;

        val
    }
}