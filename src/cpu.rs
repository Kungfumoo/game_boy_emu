//Sharp SM83 CPU
use registers::Registers;
use flags::Flags;

mod instructions;
mod registers;
mod flags;

pub struct CPU {
    registers: Registers,
    flags: Flags
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

    pub fn status(&self) {
        println!("PC: {}", self.registers.program_counter);
        println!(
            "Z: {}, N: {}, H: {}, C: {}",
            self.flags.zero,
            self.flags.subtract,
            self.flags.half_carry,
            self.flags.carry
        );
    }

    pub fn execute(&mut self, op_code: u8) {
        let change = instructions::execute(
            self,
            op_code
        );

        let pc_increment: &u16 = &change.byte_length.into();
        self.registers.program_counter += pc_increment;

        //TODO: t states

        self.flags.zero = match change.flags.zero {
            Some(state) => state,
            None => self.flags.zero
        };

        self.flags.subtract = match change.flags.subtract {
            Some(state) => state,
            None => self.flags.subtract
        };

        self.flags.half_carry = match change.flags.half_carry {
            Some(state) => state,
            None => self.flags.half_carry
        };

        self.flags.carry = match change.flags.carry {
            Some(state) => state,
            None => self.flags.carry
        };
    }
}