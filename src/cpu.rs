use std::ops::Range;
use std::time::Duration;
use std::thread;

//Sharp SM83 CPU
use registers::Registers;
use flags::Flags;
use instructions::{StateChange, get_byte_length};
use memory::Memory;

mod instructions;
mod registers;
mod flags;
mod memory;
mod util;

#[cfg(test)]
#[path = "./cpu_test.rs"]
mod cpu_test;

const CPU_SPEED_MHZ: f64 = 1e-6 * 2.0; //TODO: currently set to 2hz for testing should be 4.194304Mhz

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
        println!("PC: {:#02x}", self.registers.program_counter);
        println!("SP: {:#02x}", self.registers.stack_pointer);
        println!(
            "IME: {}",
            match self.ime {
                ImeStatus::SET => "true",
                ImeStatus::UNSET => "false",
                ImeStatus::SCHEDULED => "scheduled"
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

    //map values by bulk to memory, mem_range specifies where in memory
    pub fn memory_map(&mut self, mem_range: Range<usize>, values: Vec<u8>) {
        let mut idx = 0;
        let len = values.len();

        for addr in mem_range {
            self.memory[addr] = values[idx];
            idx += 1;

            if idx >= len {
                return;
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            let pc = self.registers.program_counter;
            let op_code = self.memory[pc as usize];

            if op_code == 0x00 {
                return;
            }

            //TODO: temp
            println!("Executing {:#02x}", op_code);

            if op_code == 0xCB {
                println!("PREFIX {:#02x}", self.memory[(pc + 1) as usize]);
            }

            if op_code == 0x67 { //TEMP: line 118 dmg.asm
                println!("hit");
            }

            if op_code == 0x0D { //TEMP: line 141 dmg.asm after breaking at 0x67, it's never reaching here.
                println!("hit");
            }

            let change = instructions::execute(
                self,
                op_code
            );

            self.registers.program_counter = pc.wrapping_add(get_byte_length(op_code) as u16);
            self.update(&change);
            //self.delay(change.t_states);
        }
    }

    fn delay(&self, t_states: u8) {
        const SPEED_HZ: f64 = CPU_SPEED_MHZ * 1e+6;
        const T_TO_M_CYCLE: u8 = 4; //Timing states divisible by 4, 4 t_states = 1 machine cycle
        const M_CYCLE_TO_SECOND: f64 = 1.0 / SPEED_HZ; //1 hz = 1 machine cycle per second

        let m_cycles = (t_states / T_TO_M_CYCLE) as f64;
        let delay = Duration::from_secs_f64(m_cycles * M_CYCLE_TO_SECOND);

        thread::sleep(delay);
    }

    fn update(&mut self, change: &StateChange) {
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