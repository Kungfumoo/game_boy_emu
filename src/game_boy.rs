use std::time::{Duration, Instant};
use std::thread;

use crate::{
    cpu::{CPU, T_TO_M_CYCLE},
    ppu::{PPU, LCD_REGISTERS}
};

const CARTRIDGE_ROM: usize = 0x7FFF;
const CPU_SPEED_MHZ: f64 = 1e-6 * 2.0; //TODO: currently set to 2hz for testing should be 4.194304Mhz
const DISPLAY_V_SYNC_HZ: f64 = 59.73;

struct TimeState {
    prev: Instant,
    delay: Duration
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
        let mut cpu_time_state = TimeState {
            prev: Instant::now(),
            delay: Duration::new(0, 0)
        };

        loop { //TODO: throttle
            cpu_time_state = self.cpu_run(cpu_time_state);
            /*self.cpu.memory_map(
                LCD_REGISTERS,
                self.ppu.step() //TODO: screen timing
            );*/
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
            delay: delay(self.cpu.step())
        }
    }
}

fn delay(t_states: u8) -> Duration {
    const SPEED_HZ: f64 = CPU_SPEED_MHZ * 1e+6;
    const M_CYCLE_TO_SECOND: f64 = 1.0 / SPEED_HZ; //1 hz = 1 machine cycle per second

    let m_cycles = (t_states / T_TO_M_CYCLE) as f64;
    Duration::from_secs_f64(m_cycles * M_CYCLE_TO_SECOND)
}

fn has_delayed(state: &TimeState) -> bool {
    let now = Instant::now() - state.prev;

    now >= state.delay
}