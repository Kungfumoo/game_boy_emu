use std::time::{Duration, Instant};
use std::thread;

use crate::{
    cpu::CPU,
    ppu::{PPU, LCD_REGISTERS}
};

const CARTRIDGE_ROM: usize = 0x7FFF;

struct TimeState {
    prev: Instant,
    delay: Duration
}

impl TimeState {
    pub fn new() -> TimeState {
        TimeState {
            prev: Instant::now(),
            delay: Duration::new(0, 0)
        }
    }
}

pub struct GameBoy {
    cpu: CPU,
    ppu: PPU
}

impl GameBoy {
    pub fn init(rom: Vec<u8>) -> GameBoy {
        let mut cpu = CPU::new();
        cpu.memory_map(
            0x0000..CARTRIDGE_ROM ,
            rom
        );

        GameBoy {
            cpu: cpu,
            ppu: PPU::init()
        }
    }

    pub fn run(&mut self) {
        let mut cpu_time_state = TimeState::new();
        let mut ppu_time_state = TimeState::new();

        loop {
            cpu_time_state = self.cpu_run(cpu_time_state);
            ppu_time_state = self.ppu_run(ppu_time_state);
            thread::sleep(cpu_time_state.delay); //TODO: currently throttling by the smallest delay (cpu) but will probs need to change
        }
    }

    pub fn status(&self) {
        self.cpu.status();
    }

    fn cpu_run(&mut self, state: TimeState) -> TimeState {
        if !has_delayed(&state) {
            return state;
        }

        TimeState {
            prev: Instant::now(),
            delay: self.cpu.step()
        }
    }

    fn ppu_run(&mut self, state: TimeState) -> TimeState {
        if !has_delayed(&state) {
            return state;
        }

        let (values, delay) = self.ppu.step(
            &self.cpu.memory_slice(LCD_REGISTERS)
        );

        self.cpu.memory_map(
            LCD_REGISTERS,
            values
        );

        TimeState {
            prev: Instant::now(),
            delay: delay
        }
    }
}

fn has_delayed(state: &TimeState) -> bool {
    let now = Instant::now() - state.prev;

    now >= state.delay
}