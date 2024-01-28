use self::{lcd_control::LcdControl, lcd_status::LcdStatus};

mod lcd_control;
mod lcd_status;

pub struct Registers {
    lcdc: u8, //LCD Control register
    stat: u8, //LCD Status register
    ly: u8 //LCD Y Coordinate
}

impl Registers {
    pub fn from_array(arr: &[u8]) -> Registers {
        Registers {
            lcdc: arr[0],
            stat: arr[1],
            ly: arr[4]
        }
    }

    pub fn increment_ly(&mut self) -> u8 {
        self.ly += 1;
        self.get_ly()
    }

    pub fn reset_ly(&mut self) {
        self.ly = 0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![
            self.lcdc, //0xFF40
            self.stat, //0xFF41
            0x00, //0xFF42
            0x00, //0xFF43
            self.ly, //0xFF44
        ]
    }

    pub fn get_ly(&self) -> u8 {
        self.ly
    }

    pub fn get_lcd_control(&self) -> LcdControl {
        LcdControl::from_byte(self.lcdc)
    }

    pub fn get_lcd_status(&self) -> LcdStatus {
        LcdStatus::from_byte(self.stat)
    }
}