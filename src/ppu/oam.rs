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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_sprite() {
        const MAX: usize = SPRITE_BYTES as usize;

        let mut memory: [u8; MAX] = [0; MAX];
        memory[0] = 4;
        memory[1] = 8;
        memory[2] = 7;
        memory[3] = 0b10100000;

        let oam = OAM {
            oam: &memory
        };

        let sprite = oam.get_sprite(0);

        assert_eq!(4, sprite.y_position);
        assert_eq!(8, sprite.x_position);
        assert_eq!(7, sprite.tile_no);
        assert!(sprite.bg_priority());
        assert!(!sprite.y_flip());
        assert!(sprite.x_flip());
        assert!(!sprite.use_obp1());
    }
}