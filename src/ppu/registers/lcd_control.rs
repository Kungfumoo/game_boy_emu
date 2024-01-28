pub struct LcdControl {
    pub bg_window_enable: bool,
    pub sprite_enable: bool,
    pub tall_sprite: bool,
    pub bg_tile_map_area: bool,
    pub bg_win_tile_data_area: bool,
    pub win_enable: bool,
    pub win_tile_map_enable: bool,
    pub lcd_ppu_enable: bool
}

impl LcdControl {
    pub fn from_byte(byte: u8) -> LcdControl {
        LcdControl {
            bg_window_enable: (byte & 0b00000001) != 0,
            sprite_enable: (byte & 0b00000010) != 0,
            tall_sprite: (byte & 0b00000100) != 0,
            bg_tile_map_area: (byte & 0b00001000) != 0,
            bg_win_tile_data_area: (byte & 0b00010000) != 0,
            win_enable: (byte & 0b00100000) != 0,
            win_tile_map_enable: (byte & 0b01000000) != 0,
            lcd_ppu_enable: (byte & 0b10000000) != 0,
        }
    }
}