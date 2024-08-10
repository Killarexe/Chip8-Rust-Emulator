mod chip8;
mod args;

use clap::Parser;

use std::{fs::read, thread, time};

use raylib::prelude::*;

use chip8::*;
use args::*;

fn main() {
    let args: Args = Args::parse();
    let (mut window, thread) = raylib::init()
        .size(1280, 720)
        .title("Chip8 emulator")
        .build();
    let mut chip8: Chip8 = Chip8::new();

    if let Ok(rom) = read(args.rom_file) {
        for (index, byte) in rom.iter().enumerate() {
            chip8.memory[index as usize + 0x200] = *byte;
        }
    }

    let threshold: f32 = 1.0 / 60.0;
    let mut current_time: f32 = 0.0;
    let pixel_size: (i32, i32) = (1280 / 64, 720 / 32);
    while !window.window_should_close() {
        let delta: f32 = window.get_frame_time();
        if current_time > threshold {
            chip8.cycle();
            current_time = 0.0;
        } else {
            current_time += delta;
        }
        let mut draw = window.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        for (index, byte) in chip8.pixel_buffer.iter().enumerate(){
            let x: i32 = (index % 64) as i32;
            let y: i32 = f32::floor(index as f32 / 64.0) as i32;
            if *byte == 1 {
                draw.draw_rectangle(x * pixel_size.0, y * pixel_size.1, pixel_size.0, pixel_size.1, Color::GREEN);
            }
        }
    }
}
