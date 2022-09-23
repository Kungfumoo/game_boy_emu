use super::instructions::FlagChange;

pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool
}

impl Flags {
    pub fn update(&mut self, change: &FlagChange) {
        if let Some(state) = change.zero {
            self.zero = state;
        }

        if let Some(state) = change.subtract {
            self.subtract = state;
        }

        if let Some(state) = change.half_carry {
            self.half_carry = state;
        }

        if let Some(state) = change.carry {
            self.carry = state;
        }
    }
}

pub fn is_half_carry_add(a: u8, b: u8) -> bool {
    (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10
}

pub fn is_half_carry_subtract(a: u8, b: u8) -> bool {
    (((a & 0xF) - (b & 0xF)) & 0x10) == 0x10
}