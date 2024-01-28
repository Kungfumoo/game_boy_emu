use self::{lcd_control::LcdControl, lcd_status::LcdStatus};

use super::Mode;

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

    pub fn update_mode(&mut self, mode: &Mode) {
        let mut stat = self.get_lcd_status();
        stat.update_mode_bits(
            match mode {
                Mode::HBlank => 0,
                Mode::VBlank => 1,
                Mode::OamScan => 2,
                Mode::Drawing => 3
            }
        );

        self.stat = stat.to_byte();
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![
            self.lcdc, //0xFF40
            self.stat | 0b10000000, //0xFF41 (bit 7 is always set to 1)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_increment_ly() {
        let mut registers = Registers::from_array(&[0; 8]);

        assert_eq!(0, registers.ly);
        registers.increment_ly();
        assert_eq!(1, registers.ly);
    }

    #[test]
    pub fn test_get_ly() {
        let mut arr = [0; 8];
        arr[4] = 10;

        let registers = Registers::from_array(&arr);

        assert_eq!(10, registers.get_ly());
    }

    #[test]
    pub fn test_reset_ly() {
        let mut arr = [0; 8];
        arr[4] = 10;

        let mut registers = Registers::from_array(&arr);

        assert_eq!(10, registers.get_ly());

        registers.reset_ly();

        assert_eq!(0, registers.get_ly());
    }

    #[test]
    pub fn test_update_mode() {
        let mut registers = Registers::from_array(&[0; 8]);

        registers.update_mode(&Mode::Drawing);

        assert_eq!(0b10000011, registers.stat);
    }
}