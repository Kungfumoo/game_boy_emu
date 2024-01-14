use self::tile::Tile;

mod tile;

pub struct VRAM<'a> { //<'a> is a lifetime parameter, telling borrow checker we're expecting the borrowed vram array reference and this struct to be in the same lifetime represented as `'a`
    pub vram: &'a[u8] //NOTE: vram is a slice of gameboy memory where 0 index is actually 0x8000 in memory
}

impl VRAM<'_> {
    pub fn get_tile(&self, tile_number: u16, signed_mode: bool) -> Tile {
        //TODO: cover signed mode calculation, eg 0xFF is actually -127
        let start_addr = {
            if signed_mode {
                0x1000
            } else {
                0x0
            }
        } + (tile_number * 16) as usize;

        Tile {
            data: &self.vram[start_addr..(start_addr + 16)]
        }
    }
}