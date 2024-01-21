use self::tile::Tile;

mod tile;

const TILE_BYTES: u16 = 16;

pub struct VRAM<'a> { //<'a> is a lifetime parameter, telling borrow checker we're expecting the borrowed vram array reference and this struct to be in the same lifetime represented as `'a`
    pub vram: &'a[u8] //NOTE: vram is a slice of gameboy memory where 0 index is actually 0x8000 in memory
}

impl VRAM<'_> {
    pub fn get_tile(&self, tile_number: u8, signed_mode: bool) -> Tile {
        let start_addr = {
            if signed_mode {
                #[allow(overflowing_literals)]
                let modifier = (tile_number as i8) as i16 * (TILE_BYTES as i16);

                (0x1000 as u16).wrapping_add_signed(modifier)
            } else {
                (tile_number as u16) * TILE_BYTES
            }
        } as usize;

        Tile {
            data: &self.vram[start_addr..(start_addr + (TILE_BYTES as usize))]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ppu::Colours;
    use super::*;

    #[test]
    pub fn test_get_tile_unsigned() {
        let mut memory: [u8; 0xFFFF] = [0; 0xFFFF];

        memory[0x00] = 0b10000000;
        memory[0x01] = 0b10000000;
        memory[0xFF0] = 0b10000000;
        memory[0xFF1] = 0b10000000;

        let vram = VRAM {
            vram: &memory
        };

        let tile = vram.get_tile(0, false);

        assert!(matches!(tile.get_pixel_colour(0, 0), Colours::Black));

        let tile = vram.get_tile(255, false);

        assert!(matches!(tile.get_pixel_colour(0, 0), Colours::Black));
    }

    #[test]
    pub fn test_get_tile_signed() {
        let mut memory: [u8; 0xFFFF] = [0; 0xFFFF];

        memory[0x1000] = 0b10000000;
        memory[0x1001] = 0b10000000;
        memory[0xFF0] = 0b10000000;
        memory[0xFF1] = 0b10000000;

        let vram = VRAM {
            vram: &memory
        };

        let tile = vram.get_tile(0, true);

        assert!(matches!(tile.get_pixel_colour(0, 0), Colours::Black));

        let tile = vram.get_tile(255, true);

        assert!(matches!(tile.get_pixel_colour(0, 0), Colours::Black));
    }
}