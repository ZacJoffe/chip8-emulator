extern crate sdl2;

use std::env;
use std::fs;

use cpu::Cpu;

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

    println!("Hello, world!");
}
