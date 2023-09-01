pub struct RegisterChange {
    pub pc: Option<u16>,
    pub sp: Option<u16>,
    pub a: Option<u8>,
    pub b: Option<u8>,
    pub c: Option<u8>,
    pub d: Option<u8>,
    pub e: Option<u8>,
    pub h: Option<u8>,
    pub l: Option<u8>
}

impl RegisterChange {
    pub fn default() -> RegisterChange {
        RegisterChange {
            pc: Option::None,
            sp: Option::None,
            a: Option::None,
            b: Option::None,
            c: Option::None,
            d: Option::None,
            e: Option::None,
            h: Option::None,
            l: Option::None
        }
    }

    //Short hand to create register change based on common opcode, eg 0xn0 = b, 0xn1 = c and so on
    pub fn create_from_opcode(opcode: u8, value: Option<u8>) -> RegisterChange {
        let index = opcode % 0x08;

        match index {
            0x00 => RegisterChange { b: value, ..RegisterChange::default() },
            0x01 => RegisterChange { c: value, ..RegisterChange::default() },
            0x02 => RegisterChange { d: value, ..RegisterChange::default() },
            0x03 => RegisterChange { e: value, ..RegisterChange::default() },
            0x04 => RegisterChange { h: value, ..RegisterChange::default() },
            0x05 => RegisterChange { l: value, ..RegisterChange::default() },
            0x07 => RegisterChange { a: value, ..RegisterChange::default() },
            _ => panic!("{:#02x} not a valid index for creating a register change from opcode", index)
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
    pub h: u8,
    pub l: u8
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            program_counter: 0x0000, //https://gbdev.io/pandocs/Power_Up_Sequence.html ~ Boot ROM at 0000 then cartrige ROM at 0100
            stack_pointer: 0x05, //TODO: may not be correct
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00
        }
    }

    pub fn update(&mut self, change: &RegisterChange) {
        if let Some(value) = change.pc {
            self.program_counter = value;
        }

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

        if let Some(value) = change.h {
            self.h = value;
        }

        if let Some(value) = change.l {
            self.l = value;
        }
    }

    pub fn bc(&self) -> u16 {
        to16_bit(self.c, self.b)
    }

    pub fn de(&self) -> u16 {
        to16_bit(self.e, self.d)
    }

    pub fn hl(&self) -> u16 {
        to16_bit(self.l, self.h)
    }

    pub fn from_opcode_index(&self, opcode: u8) -> u8 {
        let index = opcode % 0x08;

        match index {
            0x00 => self.b,
            0x01 => self.c,
            0x02 => self.d,
            0x03 => self.e,
            0x04 => self.h,
            0x05 => self.l,
            0x07 => self.a,
            _ => panic!("{:#02x} not a valid index for register fetching", index)
        }
    }
}

pub fn to16_bit(lsb: u8, msb: u8) -> u16 {
    let l16: u16 = msb.into();
    let r16: u16 = lsb.into();

    (l16 << 8) | r16
}

pub fn to8_bit(value: u16) -> (u8, u8) {
    let msb: u8 = (value >> 8) as u8;
    let lsb: u8 = (value & 0xFF) as u8;

    (lsb, msb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_from_opcode() {
        let change = RegisterChange::create_from_opcode(0x53, Some(0x0A));

        assert!(matches!(change.e, Option::Some(0x0A)));

        let change = RegisterChange::create_from_opcode(0x5B, Some(0x0B));

        assert!(matches!(change.e, Option::Some(0x0B)));
    }

    #[test]
    #[should_panic(expected = "0x6 not a valid index for creating a register change from opcode")]
    fn test_create_from_opcode_fail() {
        RegisterChange::create_from_opcode(0x06, Some(0x0A));
    }

    #[test]
    fn test_from_opcode_index() {
        let mut registers = Registers::new();
        registers.a = 0x30;

        assert_eq!(0x30, registers.from_opcode_index(0x07));
        assert_eq!(0x30, registers.from_opcode_index(0x0F));
    }

    #[test]
    #[should_panic(expected = "0x6 not a valid index for register fetching")]
    fn test_from_opcode_index_fail() {
        let registers = Registers::new();

        registers.from_opcode_index(0x06);
    }

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
        assert_eq!(registers.a, 0x00);
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
            to16_bit(0x0A, 0x01),
            0x010A
        );

        assert_eq!(
            to16_bit(0x95, 0x00),
            0x0095
        )
    }

    #[test]
    fn test_to8_bit() {
        let (lsb, msb) = to8_bit(0x010A);

        assert_eq!(0x01, msb);
        assert_eq!(0x0A, lsb);

        let (lsb, msb) = to8_bit(0xA034);

        assert_eq!(0xA0, msb);
        assert_eq!(0x34, lsb);
    }
}