use std::time::Duration;
use std::thread;

use crate::{
    cpu::{CPU, T_TO_M_CYCLE},
    ppu::{PPU, LCD_REGISTERS}
};

const CARTRIDGE_ROM: usize = 0x7FFF;
const CPU_SPEED_MHZ: f64 = 1e-6 * 2.0; //TODO: currently set to 2hz for testing should be 4.194304Mhz

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
        loop {
            let t_states = self.cpu.step();

            self.cpu.memory_map(
                LCD_REGISTERS,
                self.ppu.step() //TODO: screen timing
            );

            if t_states == 0 {
                return;
            }

            self.delay(t_states);
        }
    }

    fn delay(&self, t_states: u8) {
        const SPEED_HZ: f64 = CPU_SPEED_MHZ * 1e+6;
        const M_CYCLE_TO_SECOND: f64 = 1.0 / SPEED_HZ; //1 hz = 1 machine cycle per second

        let m_cycles = (t_states / T_TO_M_CYCLE) as f64;
        let delay = Duration::from_secs_f64(m_cycles * M_CYCLE_TO_SECOND);

        thread::sleep(delay);
    }

    pub fn status(&self) {
        self.cpu.status();
    }
}