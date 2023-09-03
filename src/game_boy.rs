use crate::{
    cpu::CPU,
    ppu::PPU
};

const CARTRIDGE_ROM: usize = 0x7FFF;

pub struct GameBoy {
    cpu: CPU,
    display: PPU
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
            display: PPU::init()
        }
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }

    pub fn status(&self) {
        self.cpu.status();
    }
}