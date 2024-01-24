use std::ops::RangeInclusive;
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
pub const DOTS_PER_M_CYCLE: u8 = 4;
pub const DISPLAY_REFRESH_RATE: f64 = 59.73;

const MAX_SCANLINES: u8 = 153;
const DOTS_PER_SCANLINE: u16 = 456;

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

enum Mode {
    HBlank,
    VBlank,
    OamScan,
    Drawing
}

pub struct PPU {
    sdl: Sdl,
    window: RendererWindow,
    texture_buffer: Texture,
    mode: Mode,
    dot_counter: u16
}

impl PPU {
    pub fn init() -> PPU {
        let sdl = Sdl::init(InitFlags::EVERYTHING);
        let window = sdl.create_renderer_window(CreateWinArgs {
            title: "Game Boy Emulator",
            width: VIEWPORT_PIXEL_WIDTH,
            height: VIEWPORT_PIXEL_HEIGHT,
            ..Default::default()
        }, RendererFlags::ACCELERATED_VSYNC);

        if let Result::Err(error) = window {
            panic!("SDL window Error: {:?}", error);
        }

        let window = window.unwrap();
        let pix_buf = [r8g8b8a8_Srgb { r: 255, g: 127, b: 16, a: 255 }; 64];
        let surface = sdl.create_surface_from(&pix_buf, 8, 8);

        if let Result::Err(error) = surface {
            panic!("SDL surface Error: {:?}", error);
        }

        let surface = surface.unwrap();
        let tex = window.create_texture_from_surface(&surface);

        if let Result::Err(error) = tex {
            panic!("SDL texture Error: {:?}", error);
        }

        PPU {
            sdl,
            window,
            texture_buffer: tex.unwrap(),
            mode: Mode::VBlank,
            dot_counter: 0
        }
    }

    //PPU cycle and return values of registers
    pub fn dot(&mut self, registers: &[u8], vram: &[u8], oam: &[u8]) -> (Vec<u8>, bool) {
        let mut registers = Registers::from_array(registers);

        self.dot_counter += 1;

        /*
            TODO: render pixel to some buffer to be used by the sdl library
            need to check how 'real time' this needs to be too.
         */

        let mut is_frame_complete = false;
        if self.dot_counter == DOTS_PER_SCANLINE {
            registers.ly += 1;

            if registers.ly == MAX_SCANLINES { //Complete Frame
                registers.ly = 0;
                is_frame_complete = true;
                self.refresh_window(&registers);
            }

            self.dot_counter = 0;
        }

        (registers.to_vec(), is_frame_complete)
    }

    fn refresh_window(&mut self, registers: &Registers) {
        while let Some((event, _)) = self.sdl.poll_events() {
            match event {
                Event::Quit => std::process::exit(0),
                _ => (),
            }
        }

        let lcdc = registers.get_lcd_control();
        let stat = registers.get_lcd_status();

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