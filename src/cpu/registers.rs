pub struct RegisterChange {
    pub sp: Option<u16>,
    pub a: Option<u8>,
    pub b: Option<u8>,
    pub c: Option<u8>,
    pub d: Option<u8>,
    pub e: Option<u8>,
    pub f: Option<u8>,
    pub h: Option<u8>,
    pub l: Option<u8>
}

impl RegisterChange {
    pub fn default() -> RegisterChange {
        RegisterChange {
            sp: Option::None,
            a: Option::None,
            b: Option::None,
            c: Option::None,
            d: Option::None,
            e: Option::None,
            f: Option::None,
            h: Option::None,
            l: Option::None
        }
    }
}

pub struct Registers {
    pub program_counter: u16,
    pub stack_pointer: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            program_counter: 0x0150, //TODO: may not be correct
            stack_pointer: 0x00,
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00
        }
    }

    pub fn update(&mut self, change: &RegisterChange) {
        //if change.sp enum value is 'Some' then update stack pointer with attached value
        if let Some(value) = change.sp {
            self.stack_pointer = value;
        }

        if let Some(value) = change.a {
            self.a = value;
        }

        if let Some(value) = change.b {
            self.b = value;
        }

        if let Some(value) = change.c {
            self.c = value;
        }

        if let Some(value) = change.d {
            self.d = value;
        }

        if let Some(value) = change.e {
            self.e = value;
        }

        if let Some(value) = change.f {
            self.f = value;
        }

        if let Some(value) = change.h {
            self.h = value;
        }

        if let Some(value) = change.l {
            self.l = value;
        }
    }

    pub fn af(&self) -> u16 {
        to16_bit(self.a, self.f)
    }

    pub fn bc(&self) -> u16 {
        to16_bit(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        to16_bit(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        to16_bit(self.h, self.l)
    }
}

pub fn to16_bit(left: u8, right: u8) -> u16 {
    let l16: u16 = left.into();
    let r16: u16 = right.into();

    (l16 << 8) | r16
}

pub fn to8_bit(value: u16) -> (u8, u8) {
    let left: u8 = (value >> 8) as u8;
    let right: u8 = (value & 0xFF) as u8;

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change() {
        let mut registers = Registers::new();

        registers.update(&RegisterChange {
            b: Some(0x04),
            c: Some(0x07),
            ..RegisterChange::default()
        });

        assert_eq!(registers.b, 0x04);
        assert_eq!(registers.c, 0x07);
    }

    #[test]
    fn test_af() {
        let mut registers = Registers::new();

        registers.a = 0x01;
        registers.f = 0x0f;

        assert_eq!(registers.af(), 0x010f);
    }

    #[test]
    fn test_bc() {
        let mut registers = Registers::new();

        registers.b = 0x01;
        registers.c = 0x0f;

        assert_eq!(registers.bc(), 0x010f);
    }

    #[test]
    fn test_de() {
        let mut registers = Registers::new();

        registers.d = 0x01;
        registers.e = 0x0f;

        assert_eq!(registers.de(), 0x010f);
    }

    #[test]
    fn test_hl() {
        let mut registers = Registers::new();

        registers.h = 0x01;
        registers.l = 0x0f;

        assert_eq!(registers.hl(), 0x010f);
    }

    #[test]
    fn test_to16_bit() {
        assert_eq!(
            to16_bit(0x01, 0x0A),
            0x010A
        );
    }

    #[test]
    fn test_to8_bit() {
        let (left, right) = to8_bit(0x010A);

        assert_eq!(0x01, left);
        assert_eq!(0x0A, right);
    }
}