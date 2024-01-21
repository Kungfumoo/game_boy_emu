use self::sprite::Sprite;

mod sprite;

const SPRITE_BYTES: u16 = 4;

pub struct OAM<'a> {
    pub oam: &'a[u8]
}

impl OAM<'_> {
    pub fn get_sprite(&self, index: u16) -> Sprite {
        let start_addr = (index * SPRITE_BYTES) as usize;
        let data = &self.oam[start_addr..(start_addr + (SPRITE_BYTES as usize))];

        Sprite {
            y_position: data[0],
            x_position: data[1],
            tile_no: data[2],
            flags: data[3]
        }
    }
}