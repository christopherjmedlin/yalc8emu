mod cpu;
mod fonts;
mod rom;
mod display;
mod keypad;

extern crate rand;
extern crate sdl2;

use std::thread::sleep;
use std::time::Duration;
use std::env;

use sdl2::event::Event;
use sdl2::pixels::Color;

fn main() {
    let args: Vec<String> = env::args().collect();

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window("Chip8", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    
    let mut cpu = cpu::Chip8::new();
    let mut rom = [0; 3583];
    rom::load_rom_file(&args[1], &mut rom);
    cpu.load_rom(&rom);

    'main: loop {
        cpu.cycle();
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        cpu.display.render(&mut canvas);
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'main },
                Event::KeyDown {..} |
                Event::KeyUp {..} => { cpu.keypad.handle_event(&event); },
                _ => {}
            }
        }
        
        // simulate ~60 hz
        sleep(Duration::from_millis(2));
    }
}
