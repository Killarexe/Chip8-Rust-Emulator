mod window;
mod chip8;
mod args;

use clap::Parser;

use std::fs::read;
use std::time::Duration;

use sdl2::{pixels::{Color, PixelFormatEnum}, render::{Texture, TextureCreator, TextureAccess}, video::WindowContext};
use window::*;
use chip8::*;
use args::*;

fn main() {
    let args: Args = Args::parse();
    let mut window: SdlWindow = SdlWindow::new("Test window", 1280, 720, Color::RGB(255, 0, 0));
    let texture_creator: TextureCreator<WindowContext> = window.get_canvas().texture_creator();
    let mut screen_texture: Texture = texture_creator.create_texture(Some(PixelFormatEnum::RGBA8888), TextureAccess::Streaming, 64, 32).unwrap();
    let mut chip8: Chip8 = Chip8::new();

    if let Ok(rom) = read(args.rom_file) {
        for (index, byte) in rom.iter().enumerate() {
            chip8.memory[index as usize + 0x200] = *byte;
        }
    }

    while !window.is_window_needs_quit() {
        chip8.cycle();
        window.update_event();
        window.prepare_render();

        let mut screen_buffer: [u8; 64 * 32 * 4] = [0u8; 64 * 32 * 4];
        for (index, byte) in chip8.pixel_buffer.iter().enumerate(){
            if *byte == 1 {
                screen_buffer[index * 4] = 0xFF;
                screen_buffer[index * 4 + 1] = 0xFF;
                screen_buffer[index * 4 + 2] = 0xFF;
                screen_buffer[index * 4 + 3] = 0xFF;
            } else {
                screen_buffer[index * 4] = 0x00;
                screen_buffer[index * 4 + 1] = 0x00;
                screen_buffer[index * 4 + 2] = 0x00;
                screen_buffer[index * 4 + 3] = 0x00;
            }
        }
        screen_texture.update(None, &screen_buffer, 64 * 4).unwrap();

        window.get_canvas_mut().copy(&screen_texture, None, None).unwrap();
        window.render();
        std::thread::sleep(Duration::new(0, 16 * 1000 * 1000));
    }
}
