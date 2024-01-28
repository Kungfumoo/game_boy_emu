pub struct LcdStatus { //NOTE: bit 7 is unused but always set to 1.
    pub lyc_eq_ly_enable: bool, //enables the "LYC=LY condition" to trigger a STAT interrupt.
    pub mode_2_enable: bool, //enables the "mode 2 condition" to trigger a STAT interrupt.
    pub mode_1_enable: bool, //enables the "mode 1 condition" to trigger a STAT interrupt.
    pub mode_0_enable: bool, //enables the "mode 0 condition" to trigger a STAT interrupt.
    pub coincidence: bool, //set by the ppu if ly==lyc
    pub ppu_mode_msb: bool, //set by the ppu
    pub ppu_mode_lsb: bool //set by the ppu
}

impl LcdStatus {
    pub fn from_byte(byte: u8) -> LcdStatus {
        LcdStatus {
            ppu_mode_lsb: (byte & 0b00000001) != 0,
            ppu_mode_msb: (byte & 0b00000010) != 0,
            coincidence: (byte & 0b00000100) != 0,
            mode_0_enable: (byte & 0b00001000) != 0,
            mode_1_enable: (byte & 0b00010000) != 0,
            mode_2_enable: (byte & 0b00100000) != 0,
            lyc_eq_ly_enable: (byte & 0b01000000) != 0,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let mut byte: u8 = 0;

        if self.ppu_mode_lsb {
            byte |= 0b00000001;
        }

        if self.ppu_mode_msb {
            byte |= 0b00000010;
        }

        if self.coincidence {
            byte |= 0b00000100;
        }

        if self.mode_0_enable {
            byte |= 0b00001000;
        }

        if self.mode_1_enable {
            byte |= 0b00010000;
        }

        if self.mode_2_enable {
            byte |= 0b00100000;
        }

        if self.lyc_eq_ly_enable {
            byte |= 0b01000000;
        }

        byte | 0b10000000
    }

    pub fn update_mode_bits(&mut self, value: u8) {
        self.ppu_mode_lsb = (value & 0b00000001) != 0;
        self.ppu_mode_msb = (value & 0b00000010) != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_from_bytes() {
        let stat = LcdStatus::from_byte(0b10101010);

        assert!(!stat.lyc_eq_ly_enable);
        assert!(stat.mode_2_enable);
        assert!(!stat.mode_1_enable);
        assert!(stat.mode_0_enable);
        assert!(!stat.coincidence);
        assert!(stat.ppu_mode_msb);
        assert!(!stat.ppu_mode_lsb);
    }

    #[test]
    pub fn test_to_byte() {
        let byte: u8 = 0b10101010;
        let stat = LcdStatus::from_byte(byte);

        assert_eq!(stat.to_byte(), byte);
    }

    #[test]
    pub fn test_mode_bits() {
        let mut stat = LcdStatus::from_byte(0b10101010);

        stat.update_mode_bits(0);
        assert_eq!(0b10101000, stat.to_byte());

        stat.update_mode_bits(1);
        assert_eq!(0b10101001, stat.to_byte());

        stat.update_mode_bits(2);
        assert_eq!(0b10101010, stat.to_byte());

        stat.update_mode_bits(3);
        assert_eq!(0b10101011, stat.to_byte());
    }
}