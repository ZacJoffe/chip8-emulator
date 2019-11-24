extern crate sdl2;

use cpu::Cpu;

mod cpu;
mod keypad;
mod graphics;

fn main() {
    let mut cpu = Cpu::new();

    println!("Hello, world!");
}
