use game_boy::GameBoy;

pub mod cpu;
pub mod game_boy;

fn main() {
    let rom: [u8; 0x7FFF] = [0x69; 0x7FFF];
    let gb = GameBoy::init(rom);

    gb.status();
}
