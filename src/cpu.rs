//Sharp SM83 CPU
use registers::Registers;
use flags::Flags;
use instructions::StateChange;
use memory::Memory;

mod instructions;
mod registers;
mod flags;
mod memory;
mod util;

#[cfg(test)]
#[path = "./cpu_test.rs"]
mod cpu_test;

pub struct CPU {
    memory: Memory,
    registers: Registers,
    flags: Flags,
    ime: bool, //interupt master enable flag - https://gbdev.io/pandocs/Interrupts.html
    ime_scheduled: bool //When EI is called to enable IME, it is only enabled after the next instruction, schedule first then set ime after next execution
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: Memory::new(),
            registers: Registers::new(),
            flags: Flags {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false
            },
            ime: false,
            ime_scheduled: false
        }
    }

    pub fn status(&self) {
        println!("\n===CPU STATUS===");
        println!("PC: {}", self.registers.program_counter);
        println!("SP: {}", self.registers.stack_pointer);
        println!("IME: {}", self.ime);
        println!(
            "==REG==\nA: {}, B: {}, C: {}, D: {}, E: {}, F: {}, H: {}, L: {}",
            self.registers.a,
            self.registers.b,
            self.registers.c,
            self.registers.d,
            self.registers.e,
            self.flags.to_u8(),
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
        println!(
            "==MEMORY==\n0xC001: {}\n0xC002: {}",
            self.memory[0xC001],
            self.memory[0xC002]
        );
        println!(
            "==STACK==\n0x0004: {}\n0x0003: {}\n0x0002: {}\n0x0001: {}\n0x0000: {}",
            self.memory[0x0004],
            self.memory[0x0003],
            self.memory[0x0002],
            self.memory[0x0001],
            self.memory[0x0000]
        );
    }

    //execute methods mainly used for testing
    pub fn execute(&mut self, op_code: u8) {
        self.execute_with_args(op_code, Option::None);
    }

    pub fn execute_with_args(&mut self, op_code: u8, args: Option<Vec<u8>>) {
        if let Option::Some(args) = args {
            let mut pc = self.registers.program_counter + 1;

            for i in &args {
                self.memory[pc as usize] = *i;
                pc += 1;
            }
        }

        let change = instructions::execute(
            self,
            op_code
        );

        self.update(&change)
    }

    fn update(&mut self, change: &StateChange) {
        self.registers.program_counter = self.registers.program_counter.wrapping_add_signed(
            change.byte_length
        );

        //TODO: t states - according to this article: https://forums.nesdev.org/viewtopic.php?t=14014 we may not need to care

        if self.ime_scheduled {
            self.ime_scheduled = false;
            self.ime = true;
        }

        if let Some(ime_set) = change.ime {
            if ime_set {
                self.ime_scheduled = true;
            } else {
                self.ime = false;
                self.ime_scheduled = false;
            }
        }

        self.registers.update(&change.register);
        self.flags.update(&change.flags);
        self.memory.update(&change.memory);
    }
}