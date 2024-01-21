use crate::{
    cpu::CPU,
    ppu::{
        PPU,
        DOTS_PER_M_CYCLE,
        LCD_REGISTERS,
        VRAM_RANGE,
        OAM_RANGE
    }
};

const CARTRIDGE_ROM: usize = 0x7FFF;

pub struct GameBoy {
    cpu: CPU,
    ppu: PPU
}

impl GameBoy {
    pub fn init(rom: Vec<u8>) -> GameBoy {
        let mut cpu = CPU::new();
        cpu.memory.memory_map(
            0x0000..=CARTRIDGE_ROM ,
            rom
        );

        GameBoy {
            cpu: cpu,
            ppu: PPU::init()
        }
    }

    pub fn run(&mut self) {
        loop {
            let m_cycles = self.cpu.step();

            self.ppu_run(m_cycles);

            /*
                TODO: need to implement delay
                the cpu speed (c), dots per frame (d) and screen refresh rate (s) are all linked:
                    - c / s = d
                    - c / d = s
                    - s * d = c

                So in theory if we throttle by refresh rate we should get precise timnigs?
                so we need to work out how long a frame ideally takes at game boy speeds.
                From there we subtract the emulators proceesing time from that value and then delay.
             */
        }
    }

    pub fn status(&self) {
        self.cpu.status();
    }

    fn ppu_run(&mut self, m_cycles: u8) {
        let memory = &mut self.cpu.memory;

        for _c in 0..(m_cycles * DOTS_PER_M_CYCLE) {
            let values = self.ppu.dot(
                memory.get_slice(LCD_REGISTERS),
                memory.get_slice(VRAM_RANGE),
                memory.get_slice(OAM_RANGE)
            );

            memory.memory_map(
                LCD_REGISTERS,
                values
            );
        }
    }
}