//Sharp SM83 CPU
use registers::Registers;
use flags::Flags;
use instructions::StateChange;

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
            "==REG==\nA: {}, B: {}, C: {}, D: {}, E: {}, F: {}, H: {}, L: {}",
            self.registers.a,
            self.registers.b,
            self.registers.c,
            self.registers.d,
            self.registers.e,
            self.registers.f,
            self.registers.h,
            self.registers.l
        );
        println!(
            "==FLAGS==\nZ: {}, N: {}, H: {}, C: {}",
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

        self.update(&change)
    }

    fn update(&mut self, change: &StateChange) {
        let pc_increment: &u16 = &change.byte_length.into();
        self.registers.program_counter += pc_increment;

        //TODO: t states

        self.registers.update(&change.register);
        self.flags.update(&change.flags);
    }
}