use std::ops::Range;

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

#[derive(Clone, Copy)]
pub enum ImeStatus {
    SET,
    UNSET,
    SCHEDULED //When EI is called to enable IME, it is only enabled after the next instruction, schedule first then set ime after next execution
}

pub struct CPU {
    memory: Memory,
    registers: Registers,
    flags: Flags,
    ime: ImeStatus, //interupt master enable flag - https://gbdev.io/pandocs/Interrupts.html
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
            ime: ImeStatus::UNSET
        }
    }

    pub fn status(&self) {
        println!("\n===CPU STATUS===");
        println!("PC: {}", self.registers.program_counter);
        println!("SP: {}", self.registers.stack_pointer);
        println!(
            "IME: {}",
            match self.ime {
                ImeStatus::SET => "true",
                ImeStatus::UNSET => "false",
                ImeStatus::SCHEDULED => "scheduled"
            }
        );
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

    //map values by bulk to memory, mem_range specifies where in memory
    pub fn memory_map(&mut self, mem_range: Range<usize>, values: Vec<u8>) {
        let mut idx = 0;

        for addr in mem_range {
            self.memory[addr] = values[idx];
            idx += 1;
        }
    }

    fn update(&mut self, change: &StateChange) {
        self.registers.program_counter = self.registers.program_counter.wrapping_add_signed(
            change.byte_length
        );

        //TODO: t states - according to this article: https://forums.nesdev.org/viewtopic.php?t=14014 we may not need to care

        if let ImeStatus::SCHEDULED = self.ime {
            self.ime = ImeStatus::SET;
        }

        if let Some(ime_set) = change.ime {
            self.ime = ime_set;
        }

        self.registers.update(&change.register);
        self.flags.update(&change.flags);
        self.memory.update(&change.memory);
    }
}