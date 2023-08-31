use std::io::prelude::*;
use std::fs::File;
use game_boy::GameBoy;

pub mod cpu;
pub mod game_boy;

const BOOT_ROM_NAME: &str = "assets/dmg.bin";

fn main() {
    let boot_rom_file = File::open(BOOT_ROM_NAME);

    if let Result::Err(error) = boot_rom_file {
        panic!("Error opening file {}, Error: {}", BOOT_ROM_NAME, error);
    }

    let mut file = boot_rom_file.unwrap();
    let mut rom: Vec<u8> = Vec::new();

    if let Err(error) = file.read_to_end(&mut rom) {
        panic!("Error reading file {}, Error: {}", BOOT_ROM_NAME, error);
    }

    let mut gb = GameBoy::init(rom);

    gb.run();
    gb.status();
}
