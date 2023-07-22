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
}

pub fn is_half_carry_add(a: u8, b: u8) -> bool {
    (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10
}

//TODO: need to test this more to see if correct. ie signed values probably not correct.
pub fn is_half_carry_subtract(a: u8, b: u8) -> bool {
    let a = a as i8 & 0xF;
    let b = b as i8 & 0xF;

    (a - b) & 0x10 == 0x10
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
    fn test_is_half_carry_add() {
        assert!(is_half_carry_add(62, 34));
        assert!(!is_half_carry_add(1, 1));
    }

    #[test]
    fn test_is_half_carry_subtract() {
        assert!(is_half_carry_subtract(30, 15));
        assert!(!is_half_carry_subtract(2, 1));
    }
}