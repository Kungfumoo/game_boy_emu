use self::lcd_control::LcdControl;

mod lcd_control;

pub struct Registers {
    lcdc: u8, //LCD Control register
    pub ly: u8 //LCD Y Coordinate (READ-ONLY)
}

impl Registers {
    pub fn from_vec(vector: &Vec<u8>) -> Registers {
        Registers {
            lcdc: vector[0],
            ly: vector[4]
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![
            self.lcdc, //0xFF40
            0x00, //0xFF41
            0x00, //0xFF42
            0x00, //0xFF43
            self.ly, //0xFF44
        ]
    }

    pub fn get_lcd_control(&self) -> LcdControl {
        LcdControl::from_byte(self.lcdc)
    }
}