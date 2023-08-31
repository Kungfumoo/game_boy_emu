use crate::cpu::CPU;

const CARTRIDGE_ROM: usize = 0x7FFF;

pub struct GameBoy {
    cpu: CPU
}

impl GameBoy {
    pub fn init(rom: Vec<u8>) -> GameBoy {
        let mut cpu = CPU::new();
        cpu.memory_map(
            0x0000..CARTRIDGE_ROM ,
            rom
        );

        GameBoy {
            cpu: cpu
        }
    }

    pub fn status(&self) {
        self.cpu.status();
    }
}