use std::time::{Duration, Instant};
use std::thread;

use crate::{
    cpu::CPU,
    ppu::{
        PPU,
        DOTS_PER_M_CYCLE,
        LCD_REGISTERS,
        VRAM_RANGE,
        OAM_RANGE,
        DISPLAY_REFRESH_RATE
    }
};

const CARTRIDGE_ROM: usize = 0x7FFF;
const SECOND_TO_MICROS: f64 = 1e+6;
const FRAME_TIME: Duration = Duration::from_micros((SECOND_TO_MICROS / DISPLAY_REFRESH_RATE) as u64);

pub struct GameBoy {
    cpu: CPU,
    ppu: PPU
}

impl GameBoy {
    pub fn init(rom: Vec<u8>) -> GameBoy {
        let mut cpu = CPU::new();
        cpu.memory.memory_map(
            0x0000..=CARTRIDGE_ROM,
            rom
        );

        GameBoy {
            cpu: cpu,
            ppu: PPU::init()
        }
    }

    /*
        the cpu speed (c), dots per frame (d) and screen refresh rate (s) are all linked:
            - c / s = d
            - c / d = s
            - s * d = c

        So in theory if we throttle by refresh rate we should get precise timnigs?
        so we need to work out how long a frame ideally takes at game boy speeds (think it's 16740ms).
        From there we subtract the emulators proceesing time from that value and then delay.
    */
    pub fn run(&mut self) {
        let mut prev = Instant::now();

        loop {
            let m_cycles = self.cpu.step();

            if let Some(now) = self.ppu_run(m_cycles, &prev) {
                prev = now;
            }
        }
    }

    pub fn status(&self) {
        self.cpu.status();
    }

    fn ppu_run(&mut self, m_cycles: u8, prev: &Instant) -> Option<Instant> {
        let memory = &mut self.cpu.memory;
        let mut result: Option<Instant> = None;

        for _c in 0..(m_cycles * DOTS_PER_M_CYCLE) {
            let (values, is_frame_complete) = self.ppu.dot(
                memory.get_slice(LCD_REGISTERS),
                memory.get_slice(VRAM_RANGE),
                memory.get_slice(OAM_RANGE)
            );

            memory.memory_map(
                LCD_REGISTERS,
                values
            );

            if is_frame_complete { //handle throttling
                let now = Instant::now() - (*prev);

                if now < FRAME_TIME {
                    thread::sleep(FRAME_TIME - now);
                }

                result = Some(Instant::now());
            }
        }

        result
    }
}