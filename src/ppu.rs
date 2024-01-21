use std::{ops::RangeInclusive, time::Duration};
use beryllium::{
    Sdl, init::InitFlags,
    video::{Texture, RendererWindow, CreateWinArgs, RendererFlags},
    events::Event
};
use pixel_formats::r8g8b8a8_Srgb;

use self::{
    registers::Registers,
    vram::VRAM,
    oam::OAM
};

mod registers;
mod vram;
mod oam;

pub const LCD_REGISTERS: RangeInclusive<usize> = 0xFF40..=0xFF4B;
pub const VRAM_RANGE: RangeInclusive<usize> = 0x8000..=0x97FF;
pub const OAM_RANGE: RangeInclusive<usize> = 0xFE00..=0xFE9F;

const LCD_Y_MAX: u8 = 153;

//const PIXEL_WIDTH: i32 = 256;
//const PIXEL_HEIGHT: i32 = 256;
const VIEWPORT_PIXEL_WIDTH: i32 = 160;
const VIEWPORT_PIXEL_HEIGHT: i32 = 144;

enum Colours {
    White,
    DarkGrey,
    LightGrey,
    Black
}

pub struct PPU {
    //SDL
    sdl: Sdl,
    window: RendererWindow,
    texture_buffer: Texture
}

impl PPU {
    pub fn init() -> PPU {
        let sdl = Sdl::init(InitFlags::EVERYTHING);
        let win = sdl.create_renderer_window(CreateWinArgs {
            title: "Game Boy Emulator",
            width: VIEWPORT_PIXEL_WIDTH,
            height: VIEWPORT_PIXEL_HEIGHT,
            ..Default::default()
        }, RendererFlags::ACCELERATED_VSYNC);

        if let Result::Err(error) = win {
            panic!("SDL window Error: {:?}", error);
        }

        let win = win.unwrap();
        let pix_buf = [r8g8b8a8_Srgb { r: 255, g: 127, b: 16, a: 255 }; 64];
        let surface = sdl.create_surface_from(&pix_buf, 8, 8);

        if let Result::Err(error) = surface {
            panic!("SDL surface Error: {:?}", error);
        }

        let surface = surface.unwrap();
        let tex = win.create_texture_from_surface(&surface);

        if let Result::Err(error) = tex {
            panic!("SDL texture Error: {:?}", error);
        }

        PPU {
            sdl,
            window: win,
            texture_buffer: tex.unwrap()
        }
    }

    //PPU cycle and return values of registers
    pub fn step(&mut self, registers: &[u8], vram: &[u8], oam: &[u8]) -> (Vec<u8>, Duration) {
        let mut registers = Registers::from_array(registers);
        registers.ly += 1;

        if registers.ly > LCD_Y_MAX {
            registers.ly = 0;
        }

        //DEBUG
        let vram = VRAM { vram };
        let oam = OAM { oam };
        let tile = vram.get_tile(0, false);
        let sprite = oam.get_sprite(39);

        println!(
            "t {}",
            match tile.get_pixel_colour(7, 7) {
                Colours::Black => "11",
                Colours::DarkGrey => "10",
                Colours::LightGrey => "01",
                Colours::White => "00"
            }
        );

        println!(
            "s x:{}, y:{}",
            sprite.x_position,
            sprite.y_position
        );
        //DEBUG

        self.refresh_window(&registers);

        (registers.to_vec(), delay())
    }

    fn refresh_window(&mut self, registers: &Registers) {
        while let Some((event, _)) = self.sdl.poll_events() {
            match event {
                Event::Quit => std::process::exit(0),
                _ => (),
            }
        }

        let lcdc = registers.get_lcd_control();

        //TODO: modify below
        self.window.set_draw_color(u8::MAX, u8::MAX, u8::MAX, u8::MAX).unwrap();
        self.window.clear().unwrap();

        self.window.set_draw_color(0, 0, 0, u8::MAX).unwrap();
        self.window.draw_lines(&[[1, 1], [50, 50], [10, 240]]).unwrap();
        self.window.draw_points(&[[60, 60], [70, 70], [80, 90]]).unwrap();
        self.window.draw_rects(&[[100, 100, 26, 15]]).unwrap();
        self.window.fill_rects(&[[150, 150, 70, 70]]).unwrap();
        self.window.copy(&self.texture_buffer, [0, 0, 8, 8], [200, 300, 8, 8]).unwrap();

        self.window.present();
    }
}

fn delay() -> Duration {
    //TODO: basic implementation until I have sorted the display: https://gbdev.io/pandocs/pixel_fifo.html#pixel-fifo
    Duration::from_micros(16740)
}