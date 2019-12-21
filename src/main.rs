extern crate sdl2;

use std::env;
use std::fs;

use cpu::Cpu;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::thread;

mod cpu;
mod keypad;
mod graphics;

fn main() {
    let mut cpu = Cpu::new();
    let args: Vec<String> = env::args().collect();

    let mut rom = String::from(&args[1]);
    rom = format!("roms/{}", rom);

    // let mut file = File::open(rom).unwrap();
    let game: Vec<u8> = fs::read(rom).expect("Couldn't find file!");

    cpu.load_game(game);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("chip8", 640, 320).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseMotion {..} => {},
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {

                }
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {

                }
                e => {
                    println!("{:?}", e);
                }
            }
        }

        canvas.clear();
        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    println!("Hello, world!");
}
