use std::ops::{
    RangeInclusive,
    Range
};
use beryllium::{
    Sdl, init::InitFlags,
    video::{Texture, RendererWindow, CreateWinArgs, RendererFlags},
    events::Event
};
use pixel_formats::r8g8b8a8_Srgb;

use self::{
    registers::Registers,
    vram::VRAM,
    oam::{sprite::Sprite, OAM}
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
const OAM_SCAN_RANGE: Range<u16> = 0..80;
const VBLANK_SCANLINE_START: u8 = MAX_SCANLINES - 10; //10 lines of vblank
const SPRITE_Y_MODIFIER: u8  = 16; //used to determine the real y position, ie sprite.y - 16 = actual location on the viewport.
const SPRITE_BUFFER_MAX: usize = 10;

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

#[derive(PartialEq)]
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
    sprite_buffer: Vec<Sprite>,
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
            sprite_buffer: Vec::new(),
            mode: Mode::VBlank,
            dot_counter: 0
        }
    }

    //PPU cycle and return values of registers
    pub fn dot(&mut self, registers: &[u8], vram: &[u8], oam: &[u8]) -> (Vec<u8>, bool) {
        let mut registers = Registers::from_array(registers);
        let mode = get_mode(registers.get_ly(), self.dot_counter);

        if mode != self.mode {
            self.mode = mode;
            registers.update_mode(&self.mode);
        }

        match self.mode {
            Mode::OamScan => self.oam_scan(OAM { oam }, &registers),
            _ => () //TODO: cover other modes
        }

        /*
            TODO: render pixel to some buffer to be used by the sdl library
            need to check how 'real time' this needs to be too.
         */

        self.dot_counter += 1;

        let mut is_frame_complete = false;
        if self.dot_counter == DOTS_PER_SCANLINE {
            let ly = registers.increment_ly();

            if ly == MAX_SCANLINES { //Complete Frame
                registers.reset_ly();
                is_frame_complete = true;
                self.refresh_window();
            }

            self.dot_counter = 0;
        }

        (registers.to_vec(), is_frame_complete)
    }

    fn refresh_window(&mut self) {
        while let Some((event, _)) = self.sdl.poll_events() {
            match event {
                Event::Quit => std::process::exit(0),
                _ => (),
            }
        }

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

    fn oam_scan(&mut self, oam: OAM, registers: &Registers) {
        if self.sprite_buffer.len() >= SPRITE_BUFFER_MAX {
            return;
        }

        let should_load = (self.dot_counter % 2) == 0; //PPU checks and loads a new entry every 2 dots
        if !should_load {
            return;
        }

        let sprite = oam.get_sprite(self.dot_counter / 2);
        if sprite.x_position == 0 { //x = 0 means hidden so don't load
            return;
        }

        let ly = registers.get_ly() + SPRITE_Y_MODIFIER;
        if ly < sprite.y_position { //we're below the sprite so don't load
            return;
        }

        let lcdc = registers.get_lcd_control();
        let sprite_height = sprite.get_y_height(lcdc.tall_sprite);

        if ly >= sprite_height { //we're above the sprite so don't load
            return;
        }

        self.sprite_buffer.push(sprite);
    }
}

fn get_mode(sline_counter: u8, dot_counter: u16) -> Mode { //TODO: test
    if sline_counter > VBLANK_SCANLINE_START {
        return Mode::VBlank;
    }

    if OAM_SCAN_RANGE.contains(&dot_counter) {
        return Mode::OamScan;
    }

    Mode::HBlank
}