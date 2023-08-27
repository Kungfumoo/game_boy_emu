use std::num::Wrapping;

pub const BINARY_BASE: u8 = 2;

//Below adders and subtracters make use of Wrapping to safely handle overflows without the program crashing
pub fn add8_bit(a: u8, b: u8) -> u8 {
    let a = Wrapping(a);
    let b = Wrapping(b);

    (a + b).0
}

pub fn add16_bit(a: u16, b: u16) -> u16 {
    let a = Wrapping(a);
    let b = Wrapping(b);

    (a + b).0
}

pub fn sub8_bit(a: u8, b: u8) -> u8 {
    let a = Wrapping(a);
    let b = Wrapping(b);

    (a - b).0
}

pub fn sub16_bit(a: u16, b: u16) -> u16 {
    let a = Wrapping(a);
    let b = Wrapping(b);

    (a - b).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add8_bit() {
        let result = add8_bit(0xFF, 0x03);

        assert_eq!(0x02, result);
    }

    #[test]
    fn test_add16_bit() {
        let result = add16_bit(0xFFFF, 0x03);

        assert_eq!(0x02, result);
    }

    #[test]
    fn test_sub8_bit() {
        let result = sub8_bit(0x00, 0x01);

        assert_eq!(0xFF, result);
    }

    #[test]
    fn test_sub16_bit() {
        let result = sub16_bit(0x00, 0x01);

        assert_eq!(0xFFFF, result);
    }
}