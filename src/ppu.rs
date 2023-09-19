use std::{ops::Range, time::Duration};
use beryllium::{
    Sdl, init::InitFlags,
    video::{Texture, RendererWindow, CreateWinArgs, RendererFlags},
    events::Event
};
use pixel_formats::r8g8b8a8_Srgb;

pub const LCD_REGISTERS: Range<usize> = 0xFF40..0xFF4B;

const LCD_Y_MAX: u8 = 153;
const PIXEL_WIDTH: i32 = 160;
const PIXEL_HEIGHT: i32 = 144;

pub struct PPU {
    //SDL
    sdl: Sdl,
    window: RendererWindow,
    texture_buffer: Texture,

    //GameBoy
    ly: u8 //LCD Y Coordinate (READ-ONLY)
}

impl PPU {
    pub fn init() -> PPU {
        let sdl = Sdl::init(InitFlags::EVERYTHING);
        let win = sdl.create_renderer_window(CreateWinArgs {
            title: "Game Boy Emulator",
            width: PIXEL_WIDTH,
            height: PIXEL_HEIGHT,
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
            texture_buffer: tex.unwrap(),
            ly: 0
        }
    }

    //PPU cycle and return values of registers
    pub fn step(&mut self) -> (Vec<u8>, Duration) {
        self.ly += 1;

        if self.ly > LCD_Y_MAX {
            self.ly = 0;
        }

        self.refresh_window();

        (self.sync_to_memory(), delay())
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

    fn sync_to_memory(&self) -> Vec<u8> {
        vec![
            0x00, //0xFF40
            0x00, //0xFF41
            0x00, //0xFF42
            0x00, //0xFF43
            self.ly, //0xFF44
        ]
    }
}

fn delay() -> Duration {
    //TODO: basic implementation until I have sorted the display: https://gbdev.io/pandocs/pixel_fifo.html#pixel-fifo
    Duration::from_micros(16740)
}