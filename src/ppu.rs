use crate::cpu::memory::{MemoryChange, MemoryEdit};

const LCD_Y_MAX: u8 = 153;

pub struct PPU {
    ly: u8 //LCD Y Coordinate (READ-ONLY)
}

impl PPU {
    pub fn init() -> PPU {
        PPU {
            ly: 0
        }
    }

    pub fn sync_to_memory(&self) -> MemoryChange {
        MemoryChange {
            changes: vec![
                MemoryEdit {
                    key: 0xFF44,
                    value: self.ly
                }
            ]
        }
    }

    pub fn step(&mut self) {
        self.ly += 1;

        if self.ly > LCD_Y_MAX {
            self.ly = 0;
        }
    }
}