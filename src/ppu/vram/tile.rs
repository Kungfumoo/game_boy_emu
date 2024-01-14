use crate::ppu::Colours;

pub struct Tile<'a> {
    pub data: &'a[u8]
}

impl Tile<'_> {
    pub fn get_pixel_colour(&self, x: u8, y: u8) -> Colours {
        let addr = (y * 2) as usize;
        let bit = 0b10000000 >> x;
        let first = (bit & self.data[addr]) != 0;
        let second = (bit & self.data[addr + 1]) != 0;

        if first && second {
            Colours::Black
        } else if first {
            Colours::DarkGrey
        } else if second {
            Colours::LightGrey
        } else {
            Colours::White
        }
    }
}