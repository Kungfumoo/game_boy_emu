pub struct Registers {
    pub ly: u8 //LCD Y Coordinate (READ-ONLY)
}

impl Registers {
    pub fn from_vec(vector: &Vec<u8>) -> Registers {
        Registers {
            ly: vector[4]
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![
            0x00, //0xFF40
            0x00, //0xFF41
            0x00, //0xFF42
            0x00, //0xFF43
            self.ly, //0xFF44
        ]
    }
}