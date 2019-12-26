extern crate sdl2;

use std::env;
use std::fs;

use cpu::Cpu;

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

    // if the rom isn't fond, then load pong2.c8
    let game = fs::read(rom);
    let game = match game {
        Ok(g) => g,
        Err(_) => {
            println!("Couldn't find file! Loading pong2.c8...");

            // attempt to load pong2.c8, panic if not found
            let pong = fs::read("roms/pong2.c8");
            match pong {
                Ok(p) => p,
                Err(err) => {
                    panic!("Couldn't find pong2.c8: {}", err);
                }
            }
        }
    };

    // load the game into the cpu's ram
    cpu.load_game(game);

    // initialize sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("chip8", 640, 320).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // game loop, each iteration represents a cycle of the cpu
    'running: loop {
        // match events
        for event in event_pump.poll_iter() {
            match event {
                // quit
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,

                // key pressed (set)
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => cpu.key.set(Keycode::Num1),
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => cpu.key.set(Keycode::Num2),
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => cpu.key.set(Keycode::Num3),
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => cpu.key.set(Keycode::Num4),
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => cpu.key.set(Keycode::Q),
                Event::KeyDown { keycode: Some(Keycode::W), .. } => cpu.key.set(Keycode::W),
                Event::KeyDown { keycode: Some(Keycode::E), .. } => cpu.key.set(Keycode::E),
                Event::KeyDown { keycode: Some(Keycode::R), .. } => cpu.key.set(Keycode::R),
                Event::KeyDown { keycode: Some(Keycode::A), .. } => cpu.key.set(Keycode::A),
                Event::KeyDown { keycode: Some(Keycode::S), .. } => cpu.key.set(Keycode::S),
                Event::KeyDown { keycode: Some(Keycode::D), .. } => cpu.key.set(Keycode::D),
                Event::KeyDown { keycode: Some(Keycode::F), .. } => cpu.key.set(Keycode::F),
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => cpu.key.set(Keycode::Z),
                Event::KeyDown { keycode: Some(Keycode::X), .. } => cpu.key.set(Keycode::X),
                Event::KeyDown { keycode: Some(Keycode::C), .. } => cpu.key.set(Keycode::C),
                Event::KeyDown { keycode: Some(Keycode::V), .. } => cpu.key.set(Keycode::V),

                // key lifted (reset)
                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => cpu.key.reset(Keycode::Num1),
                Event::KeyUp { keycode: Some(Keycode::Num2), .. } => cpu.key.reset(Keycode::Num2),
                Event::KeyUp { keycode: Some(Keycode::Num3), .. } => cpu.key.reset(Keycode::Num3),
                Event::KeyUp { keycode: Some(Keycode::Num4), .. } => cpu.key.reset(Keycode::Num4),
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => cpu.key.reset(Keycode::Q),
                Event::KeyUp { keycode: Some(Keycode::W), .. } => cpu.key.reset(Keycode::W),
                Event::KeyUp { keycode: Some(Keycode::E), .. } => cpu.key.reset(Keycode::E),
                Event::KeyUp { keycode: Some(Keycode::R), .. } => cpu.key.reset(Keycode::R),
                Event::KeyUp { keycode: Some(Keycode::A), .. } => cpu.key.reset(Keycode::A),
                Event::KeyUp { keycode: Some(Keycode::S), .. } => cpu.key.reset(Keycode::S),
                Event::KeyUp { keycode: Some(Keycode::D), .. } => cpu.key.reset(Keycode::D),
                Event::KeyUp { keycode: Some(Keycode::F), .. } => cpu.key.reset(Keycode::F),
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => cpu.key.reset(Keycode::Z),
                Event::KeyUp { keycode: Some(Keycode::X), .. } => cpu.key.reset(Keycode::X),
                Event::KeyUp { keycode: Some(Keycode::C), .. } => cpu.key.reset(Keycode::C),
                Event::KeyUp { keycode: Some(Keycode::V), .. } => cpu.key.reset(Keycode::V),
                _ => {}
            }
        }

        // emulate, draw, and sleep
        cpu.emulate_cycle();
        cpu.graphics.draw(&mut canvas);
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // 60 Hz
    }
}
