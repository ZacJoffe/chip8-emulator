extern crate rand;

use rand::Rng;

use crate::keypad::Keypad;
use crate::graphics::Graphics;

pub struct Cpu {
    i: u16,
    v: [u8; 16],
    pc: u16,
    sp: u16,
    stack: [u16; 16],
    mem: [u8; 4096],
    sound_timer: u8,
    delay_timer: u8,
    opcode: u16,
    pub key: Keypad,
    pub graphics: Graphics
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            i: 0x200,
            v: [0; 16],
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            mem: [0; 4096],
            sound_timer: 0,
            delay_timer: 0,
            opcode: 0,
            key: Keypad::new(),
            graphics: Graphics::new()
        };

        for i in 0..80 {
            cpu.mem[0x50 + i] = FONTSET[i];
        }

        cpu
    }

    pub fn load_game(&mut self, game: Vec<u8>) {
        // let mut data = vec![0; 0x200];
        let mut data = Vec::new();
        // load data vector with program
        for byte in game {
            data.push(byte);
        }

        for (i, &byte) in data.iter().enumerate() {
            self.mem[i + 0x200] = byte;
        }
    }

    pub fn emulate_cycle(&mut self) {
        // fetch
        self.opcode = (self.mem[self.pc as usize] as u16) << 8 | (self.mem[(self.pc as usize) + 1] as u16);

        println!("{:x}", self.opcode);
        // match first nibble of opcode for instruction
        match self.opcode & 0xf000 {
            0x0000 => self.instr_0(),
            0x1000 => self.instr_1(),
            0x2000 => self.instr_2(),
            0x3000 => self.instr_3(),
            0x4000 => self.instr_4(),
            0x5000 => self.instr_5(),
            0x6000 => self.instr_6(),
            0x7000 => self.instr_7(),
            0x8000 => self.instr_8(),
            0x9000 => self.instr_9(),
            0xa000 => self.instr_a(),
            0xb000 => self.instr_b(),
            0xc000 => self.instr_c(),
            0xd000 => self.instr_d(),
            0xe000 => self.instr_e(),
            0xf000 => self.instr_f(),
            _ => self.nop()
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }

            self.sound_timer -= 1;
        }
    }

    fn instr_0(&mut self) {
        match self.opcode & 0x00ff {
            0xe0 => {
                self.graphics.clear();
            }
            0xee => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            _ => {
                self.nop();
            }
        }

        self.pc += 2;
    }

    fn instr_1(&mut self) {
        self.pc = self.opcode_nnn();
    }

    fn instr_2(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.opcode_nnn();
    }

    fn instr_3(&mut self) {
        if self.v[self.opcode_x()] == self.opcode_nn() {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }
fn instr_4(&mut self) {
        if self.v[self.opcode_x()] != self.opcode_nn() {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn instr_5(&mut self) {
        if self.v[self.opcode_x()] == self.v[self.opcode_y()] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn instr_6(&mut self) {
        self.v[self.opcode_x()] = self.opcode_nn();
        self.pc += 2;
    }

    fn instr_7(&mut self) {
        // self.v[self.opcode_x()] += self.opcode_nn();
        // self.v[self.opcode_x()] = ((self.v[self.opcode_x()] as u16 + self.opcode_nn() as u16) & 0xff) as u8;
        self.v[self.opcode_x()] = self.v[self.opcode_x()].wrapping_add(self.opcode_nn());
        self.pc += 2;
    }

    fn instr_8(&mut self) {
        match self.opcode & 0x000f {
            0 => {
                self.v[self.opcode_x()] = self.v[self.opcode_y()];
            }
            1 => {
                self.v[self.opcode_x()] = self.v[self.opcode_x()] | self.v[self.opcode_y()];
            }
            2 => {
                self.v[self.opcode_x()] = self.v[self.opcode_x()] & self.v[self.opcode_y()];
            }
            3 => {
                self.v[self.opcode_x()] = self.v[self.opcode_x()] ^ self.v[self.opcode_y()];
            }
            4 => {
                // check for overflow/carry
                if self.v[self.opcode_x()] > 0xff - self.v[self.opcode_y()] {
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }

                /*
                if self.v[self.opcode_x()] < self.v[self.opcode_y()] {
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }
                */

                // self.v[self.opcode_x()] = ((self.v[self.opcode_x()] as u16 + self.v[self.opcode_y()] as u16) & 0xff) as u8;
                self.v[self.opcode_x()] = self.v[self.opcode_x()].wrapping_add(self.v[self.opcode_y()]);
            }
            5 => {
                // check for underflow/borrow
                if self.v[self.opcode_y()] > self.v[self.opcode_x()] {
                    self.v[0xf] = 0;
                } else {
                    self.v[0xf] = 1;
                }

                // self.v[self.opcode_x()] = ((self.v[self.opcode_x()] as u16 - self.v[self.opcode_y()] as u16) & 0xff) as u8;
                // self.v[self.opcode_x()] -= self.v[self.opcode_y()];
                self.v[self.opcode_x()] = self.v[self.opcode_x()].wrapping_sub(self.v[self.opcode_y()]);
            }
            6 => {
                // store the lsb of v[x] into v[0xf] before shifting
                self.v[0xf] = self.v[self.opcode_x()] & 0x1;
                self.v[self.opcode_x()] >>= 1;
            }
            7 => {
                // check for underflow/borrow
                if self.v[self.opcode_x()] > self.v[self.opcode_y()] {
                    self.v[0xf] = 0;
                } else {
                    self.v[0xf] = 1;
                }

                // self.v[self.opcode_x()] = self.v[self.opcode_y()] - self.v[self.opcode_x()];
                self.v[self.opcode_x()] = self.v[self.opcode_y()].wrapping_sub(self.v[self.opcode_x()]);
            }
            0xe => {
                // store the msb of v[x] into v[0xf] before shifting
                self.v[0xf] = self.v[self.opcode_x()] >> 7;
                self.v[self.opcode_x()] <<= 1;
            }
            _ => {
                self.nop();
            }
        }

        self.pc += 2;
    }

    fn instr_9(&mut self) {
        if self.v[self.opcode_x()] != self.v[self.opcode_y()] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn instr_a(&mut self) {
        self.i = self.opcode_nnn();
        self.pc += 2;
    }

    fn instr_b(&mut self) {
        self.pc = (self.v[0] as u16) + self.opcode_nnn();
    }

    fn instr_c(&mut self) {
        // let random_num: u8 = rand::thread_rng().gen();
        let mut rng = rand::thread_rng();
        let random_num: u8 = rng.gen();
        self.v[self.opcode_x()] = random_num & self.opcode_nn();

        self.pc += 2;
    }

    fn instr_d(&mut self) {
        let x = self.opcode_x();
        let y = self.opcode_y();
        let n = self.opcode_n();

        self.v[15] = self.graphics.update(self.v[x] as usize, self.v[y] as usize, n, self.i, self.mem);

        self.pc += 2;
    }

    fn instr_e(&mut self) {
        match (self.opcode & 0x00ff) as u8 {
            0x9e => {
                if self.key.is_pressed(self.v[self.opcode_x()] as usize) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xa1 => {
                if !self.key.is_pressed(self.v[self.opcode_x()] as usize) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            _ => {
                self.nop();
                self.pc += 2;
            }
        }
    }

    fn instr_f(&mut self) {
        match self.opcode & 0x00ff {
            0x07 => {
                self.v[self.opcode_x()] = self.delay_timer;
            }
            0x0a => {
                let mut pressed: bool = false;
                for i in 0..16 {
                    if self.key.is_pressed(i as usize) {
                        self.v[self.opcode_x()] = i;
                        pressed = true;
                    }
                }

                if !pressed {
                    self.pc -= 2;
                }
            }
            0x15 => {
                self.delay_timer = self.v[self.opcode_x()];
            }
            0x18 => {
                self.sound_timer = self.v[self.opcode_x()];
            }
            0x1e => {
                if self.v[self.opcode_x()] as u16 + self.i > 0x0fff {
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }

                // self.i += self.v[self.opcode_x()] as u16;
                self.i = self.i.wrapping_add(self.v[self.opcode_x()] as u16);
            }
            0x29 => {
                self.i = (self.v[self.opcode_x()] as u16 * 5) + 0x50;
            }
            0x33 => {
                self.mem[self.i as usize] = self.v[self.opcode_x() as usize] / 100;
                self.mem[(self.i + 1) as usize] = (self.v[self.opcode_x() as usize] / 10) % 10;
                // self.mem[(self.i + 2) as usize] = (self.v[self.opcode_x() as usize] % 100) % 10;
                self.mem[(self.i + 2) as usize] = self.v[self.opcode_x() as usize] % 10;
            }
            0x55 => {
                // reg dump into memory
                /*
                for i in 0..(self.opcode_x() + 1) {
                    self.mem[i + self.i as usize] = self.v[i];
                }
                */
                for i in 0..self.opcode_x() {
                    self.mem[i + self.i as usize] = self.v[i];
                }

                self.i += self.opcode_x() as u16 + 1;
            }
            0x65 => {
                // dump memory to registers
                for i in 0..(self.opcode_x() + 1) {
                    self.v[i] = self.mem[i + self.i as usize];
                }

                self.i += self.opcode_x() as u16 + 1;
            }
            _ => {
                self.nop();
            }
        }

        self.pc += 2;
    }

    // nop if instruction isn't valid
    fn nop(&self) {
        println!("Nop instruction");
    }

    // get x index from opcode
    fn opcode_x(&self) -> usize {
        ((self.opcode & 0x0f00) >> 8) as usize
    }

    // get y index from opcode
    fn opcode_y(&self) -> usize {
        ((self.opcode & 0x00f0) >> 4) as usize
    }

    fn opcode_n(&self) -> u8 {
        (self.opcode & 0x000f) as u8
    }

    fn opcode_nn(&self) -> u8 {
        (self.opcode & 0x00ff) as u8
    }

    fn opcode_nnn(&self) -> u16 {
        (self.opcode & 0x0fff) as u16
    }
}

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
