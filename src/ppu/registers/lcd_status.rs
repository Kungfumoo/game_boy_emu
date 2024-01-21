pub struct LcdStatus { //NOTE: bit 7 is unused but always set to 1.
    lyc_eq_ly_enable: bool, //enables the "LYC=LY condition" to trigger a STAT interrupt.
    mode_2_enable: bool, //enables the "mode 2 condition" to trigger a STAT interrupt.
    mode_1_enable: bool, //enables the "mode 1 condition" to trigger a STAT interrupt.
    mode_0_enable: bool, //enables the "mode 0 condition" to trigger a STAT interrupt.
    coincidence: bool, //set by the ppu if ly==lyc
    ppu_mode_msb: bool,
    ppu_mode_lsb: bool
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
}