pub struct Registers {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0x00,
            sp: 0x00,
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

    pub fn program_counter(&self) -> u16 {
        self.pc
    }

    pub fn stack_pointer(&self) -> u16 {
        self.sp
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

fn to16_bit(left: u8, right: u8) -> u16 {
    let l16: u16 = left.into();
    let r16: u16 = right.into();

    (l16 << 8) | r16
}