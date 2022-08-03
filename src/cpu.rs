//Sharp SM83 CPU
use registers::Registers;

mod registers;

pub struct CPU {
    pub registers: Registers
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new()
        }
    }
}