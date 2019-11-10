extern crate sdl2;

use cpu::Cpu;

mod cpu;

fn main() {
    let mut cpu = Cpu::new();

    println!("Hello, world!");
}
