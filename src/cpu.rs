//Sharp SM83 CPU
use registers::{Registers, PC_START};
use flags::Flags;
use instructions::{StateChange, get_byte_length};
use memory::Memory;

mod memory;
mod instructions;
mod registers;
mod flags;
mod util;

#[cfg(test)]
#[path = "./cpu_test.rs"]
mod cpu_test;

const CPU_SPEED_HZ: f64 = 4.194304 * 1e+6;
const T_TO_M_CYCLE: u8 = 4; //Timing states divisible by 4, 4 t_states = 1 machine cycle

#[derive(Clone, Copy)]
enum ImeStatus {
    Set,
    Unset,
    Scheduled //When EI is called to enable IME, it is only enabled after the next instruction, schedule first then set ime after next execution
}

pub struct CPU {
    pub memory: Memory,
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
            ime: ImeStatus::Unset
        }
    }

    pub fn status(&self) {
        println!("\n===CPU STATUS===");
        println!("PC: {:#02x}", self.registers.program_counter);
        println!("SP: {:#02x}", self.registers.stack_pointer);
        println!(
            "IME: {}",
            match self.ime {
                ImeStatus::Set => "true",
                ImeStatus::Unset => "false",
                ImeStatus::Scheduled => "scheduled"
            }
        );
        println!(
            "==REG==\nA: {:#02x}, B: {:#02x}, C: {:#02x}, D: {:#02x}, E: {:#02x}, F: {:#02x}, H: {:#02x}, L: {:#02x}",
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
            "==MEMORY==\n0xC001: {:#02x}\n0xC002: {:#02x}",
            self.memory[0xC001],
            self.memory[0xC002]
        );
        println!(
            "==STACK==\n0x0004: {:#02x}\n0x0003: {:#02x}\n0x0002: {:#02x}\n0x0001: {:#02x}\n0x0000: {:#02x}",
            self.memory[0x0004],
            self.memory[0x0003],
            self.memory[0x0002],
            self.memory[0x0001],
            self.memory[0x0000]
        );
    }

    //execute methods used for testing instructions in isolation
    pub fn execute(&mut self, op_code: u8) {
        self.execute_with_args(op_code, Option::None);
    }

    pub fn execute_with_args(&mut self, op_code: u8, args: Option<Vec<u8>>) {
        let pc = self.registers.program_counter;

        if let Option::Some(args) = args {
            let mut pc = pc + 1;

            for i in &args {
                self.memory[pc as usize] = *i;
                pc += 1;
            }
        }

        let change = instructions::execute(
            self,
            op_code
        );

        self.registers.program_counter = pc.wrapping_add(get_byte_length(op_code) as u16);
        self.update(&change)
    }

    //perform a fetch-execute cycle and return the m_cycles based on t_states
    pub fn step(&mut self) -> u8 {
        let pc = self.registers.program_counter;
        let op_code = self.memory[pc as usize];

        let change = instructions::execute(
            self,
            op_code
        );

        self.registers.program_counter = pc.wrapping_add(get_byte_length(op_code) as u16);
        self.update(&change);

        let mut t_states = change.t_states;

        if pc == PC_START { //emulate initial fetch that is not overlapped (2 cycles min)
            t_states += T_TO_M_CYCLE;
        }

        t_states / T_TO_M_CYCLE
    }

    fn update(&mut self, change: &StateChange) {
        if let ImeStatus::Scheduled = self.ime {
            self.ime = ImeStatus::Set;
        }

        if let Some(ime_set) = change.ime {
            self.ime = ime_set;
        }

        self.registers.update(&change.register);
        self.flags.update(&change.flags);
        self.memory.update(&change.memory);
    }
}