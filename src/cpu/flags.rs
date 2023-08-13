use super::util::{add8_bit, sub8_bit, add16_bit, sub16_bit};

pub struct FlagChange {
    pub zero: Option<bool>,
    pub subtract: Option<bool>,
    pub half_carry: Option<bool>,
    pub carry: Option<bool>
}

impl FlagChange {
    pub fn default() -> FlagChange {
        FlagChange {
            zero: Option::None,
            subtract: Option::None,
            half_carry: Option::None,
            carry: Option::None
        }
    }

    pub fn reset() -> FlagChange {
        FlagChange {
            zero: Option::Some(false),
            subtract: Option::Some(false),
            half_carry: Option::Some(false),
            carry: Option::Some(false),
        }
    }

    pub fn from_u8(f: u8) -> FlagChange {
        FlagChange {
            zero: Option::Some((f & 0b10000000) != 0),
            subtract: Option::Some((f & 0b01000000) != 0),
            half_carry: Option::Some((f & 0b00100000) != 0),
            carry: Option::Some((f & 0b00010000) != 0),
        }
    }
}

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

    pub fn to_u8(&self) -> u8 {
        let zero = self.zero as u8;
        let subtract = self.subtract as u8;
        let half_carry = self.half_carry as u8;
        let carry = self.carry as u8;

        zero << 7 | subtract << 6 | half_carry << 5 | carry << 4
    }
}

pub fn is_half_carry_add(a: u8, b: u8) -> bool {
    (add8_bit(a & 0xF, b & 0xF) & 0x10) == 0x10
}

pub fn is_half_carry_subtract(a: u8, b: u8) -> bool {
    (sub8_bit(a & 0xF, b & 0xF) & 0x10) == 0x10
}

pub fn is_carry_add(a: u8, b: u8) -> bool {
    let a = a as u16 & 0xFF;
    let b = b as u16 & 0xFF;

    (add16_bit(a, b) & 0x100) == 0x100
}

pub fn is_carry_subtract(a: u8, b: u8) -> bool {
    let a = a as u16 & 0xFF;
    let b = b as u16 & 0xFF;

    (sub16_bit(a, b) & 0x100) == 0x100
}

pub fn is_half_carry_add_16(a: u16, b: u16) -> bool {
    let a = a & 0xFF;
    let b = b & 0xFF;

    (add16_bit(a, b) & 0x100) == 0x100
}

pub fn is_half_carry_subtract_16(a: u16, b: u16) -> bool {
    let a = a & 0xFF;
    let b = b & 0xFF;

    (sub16_bit(a, b) & 0x100) == 0x100
}

pub fn is_carry_add_16(a: u16, b: u16) -> bool {
    let a = a as u32 & 0xFFF;
    let b = b as u32 & 0xFFF;

    (a.wrapping_add(b) & 0x1000) == 0x1000
}

pub fn is_carry_subtract_16(a: u16, b: u16) -> bool {
    let a = a as u32 & 0xFFF;
    let b = b as u32 & 0xFFF;

    (a.saturating_add(b) & 0x1000) == 0x1000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut flags = Flags {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false
        };

        flags.update(&FlagChange {
            zero: Some(true),
            ..FlagChange::default()
        });

        assert_eq!(flags.zero, true);
        assert_eq!(flags.subtract, false);
    }

    #[test]
    fn test_to_u8() {
        let flags = Flags {
            zero: true,
            subtract: false,
            half_carry: true,
            carry: false
        };

        assert_eq!(
            0b10100000,
            flags.to_u8()
        );
    }

    #[test]
    fn test_reset() {
        let mut flags = Flags {
            zero: true,
            subtract: false,
            half_carry: false,
            carry: false
        };

        flags.update(&FlagChange::reset());

        assert_eq!(flags.zero, false);
        assert_eq!(flags.subtract, false);
    }

    #[test]
    fn test_from_u8() {
        let mut flags = Flags {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false
        };

        flags.update(&FlagChange::from_u8(0b11000000));

        assert!(flags.zero);
        assert!(flags.subtract);
        assert!(!flags.half_carry);
        assert!(!flags.carry);
    }

    #[test]
    fn test_is_half_carry_add() {
        assert!(is_half_carry_add(62, 34));
        assert!(!is_half_carry_add(1, 1));
    }

    #[test]
    fn test_is_half_carry_subtract() {
        assert!(is_half_carry_subtract(30, 15));
        assert!(!is_half_carry_subtract(2, 1));
    }

    #[test]
    fn test_is_carry_add() {
        assert!(is_carry_add(140, 127));
        assert!(!is_carry_add(2, 1));
    }

    #[test]
    fn test_is_carry_subtract() {
        assert!(is_carry_subtract(230, 255));
        assert!(!is_carry_subtract(2, 1));
    }

    #[test]
    fn test_is_half_carry_add_16() {
        assert!(is_half_carry_add_16(140, 127));
        assert!(!is_half_carry_add_16(2, 1));
    }

    #[test]
    fn test_is_half_carry_subtract_16() {
        assert!(is_half_carry_subtract_16(230, 255));
        assert!(!is_half_carry_subtract_16(2, 1));
    }

    #[test]
    fn test_is_carry_add_16() {
        assert!(is_carry_add_16(65535, 1));
        assert!(!is_carry_add_16(6000, 443));
    }

    #[test]
    fn test_is_carry_subtract_16() {
        assert!(is_carry_subtract_16(65535, 63000));
        assert!(!is_carry_subtract_16(2, 1));
    }
}