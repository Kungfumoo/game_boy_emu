pub struct Sprite {
    pub y_position: u8,
    pub x_position: u8,
    pub tile_no: u8,
    pub flags: u8
}

impl Sprite {
    pub fn bg_priority(&self) -> bool {
        (self.flags & 0b10000000) != 0
    }

    pub fn y_flip(&self) -> bool {
        (self.flags & 0b01000000) != 0
    }

    pub fn x_flip(&self) -> bool {
        (self.flags & 0b00100000) != 0
    }

    pub fn use_obp1(&self) -> bool {
        (self.flags & 0b00010000) != 0
    }
}