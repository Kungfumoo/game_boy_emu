use super::instructions::RegisterChange;

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

fn to16_bit(left: u8, right: u8) -> u16 {
    let l16: u16 = left.into();
    let r16: u16 = right.into();

    (l16 << 8) | r16
}