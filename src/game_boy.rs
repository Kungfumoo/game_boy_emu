use std::time::{Duration, Instant};
use std::thread;

use crate::{
    cpu::{CPU, T_TO_M_CYCLE},
    ppu::{PPU, LCD_REGISTERS}
};

const CARTRIDGE_ROM: usize = 0x7FFF;
const CPU_SPEED_MHZ: f64 = 1e-6 * 2.0; //TODO: currently set to 2hz for testing should be 4.194304Mhz

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
            delay: cpu_delay(self.cpu.step())
        }
    }

    fn ppu_run(&mut self, state: TimeState) -> TimeState {
        if !has_delayed(&state) {
            return state;
        }

        self.cpu.memory_map(
            LCD_REGISTERS,
            self.ppu.step()
        );

        TimeState {
            prev: Instant::now(),
            delay: ppu_delay()
        }
    }
}

fn cpu_delay(t_states: u8) -> Duration {
    const SPEED_HZ: f64 = CPU_SPEED_MHZ * 1e+6;
    const M_CYCLE_TO_SECOND: f64 = 1.0 / SPEED_HZ; //1 hz = 1 machine cycle per second

    let m_cycles = (t_states / T_TO_M_CYCLE) as f64;
    Duration::from_secs_f64(m_cycles * M_CYCLE_TO_SECOND)
}

fn ppu_delay() -> Duration {
    //TODO: basic implementation until I have sorted the display: https://gbdev.io/pandocs/pixel_fifo.html#pixel-fifo
    Duration::from_micros(16740)
}

fn has_delayed(state: &TimeState) -> bool {
    let now = Instant::now() - state.prev;

    now >= state.delay
}