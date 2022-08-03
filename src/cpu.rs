//Sharp SM83 CPU
use registers::Registers;
use flags::Flags;

mod registers;
mod flags;

pub struct CPU {
    pub registers: Registers,
    pub flags: Flags
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            flags: Flags {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false
            }
        }
    }
}