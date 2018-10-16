mod cpu;
mod fonts;
mod rom;
mod display;

extern crate rand;
extern crate sdl2;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("Chip8", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    
    sleep(Duration::from_millis(20000));
    let mut chip = cpu::Chip8::new();
}
