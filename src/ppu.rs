use std::ops::Range;

pub const LCD_REGISTERS: Range<usize> = 0xFF40..0xFF4B;
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

    //PPU cycle and return values of registers
    pub fn step(&mut self) -> Vec<u8> {
        self.ly += 1;

        if self.ly > LCD_Y_MAX {
            self.ly = 0;
        }

        self.sync_to_memory()
    }

    fn sync_to_memory(&self) -> Vec<u8> {
        vec![
            0x00, //0xFF40
            0x00, //0xFF41
            0x00, //0xFF42
            0x00, //0xFF43
            self.ly, //0xFF44
        ]
    }
}