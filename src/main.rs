extern crate sdl2;

use cpu::Cpu;

mod cpu;
mod keypad;

fn main() {
    let mut cpu = Cpu::new();

    println!("Hello, world!");
}
