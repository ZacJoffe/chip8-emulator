# Chip8 Emulator

A Chip8 emulator written in Rust.

![](assets/pong_screenshot.png?raw=true)

## About

I've always been interested in emulation, and I thought this project would be a
great way to learn more about it and dive head-first into Rust.

Two indispensable resources that made this project possible were the Chip8
[Wikipedia article](https://en.wikipedia.org/wiki/CHIP-8) and this [blog
post](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/).

## Dependencies

Other than `cargo`, the only requirement for compiling is `SDL2`.

## Build

Simply run:
```
cargo run [romname].c8
```
Replace `romname` with one of the provided roms in the `roms` folder. If the rom
isn't found or no arguments are given, the emulator will default to loading `pong2.c8`.
