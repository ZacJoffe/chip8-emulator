extern crate rand;

use rand::Rng;

pub struct Cpu {
    i: u16,
    v: [u8; 16],
    pc: u16,
    sp: u16,
    stack: [u16; 16],
    mem: [u8; 4096],
    sound_timer: u8,
    delay_timer: u8,
    opcode: u16
}

impl Cpu {
    pub fn new () -> Cpu {
        let mut cpu = Cpu {
            i: 0x200,
            v: [0; 16],
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            mem: [0; 4096],
            sound_timer: 0,
            delay_timer: 0,
            opcode: 0
        };

        for i in 0..80 {
            cpu.mem[i] = FONTSET[i];
        }

        cpu
    }

    fn load_game(&mut self, game: Vec<u8>) {
        let mut data = vec![0; 0x200];
        // load data vector with program
        for byte in game {
            data.push(byte);
        }

        for (i, &byte) in data.iter().enumerate() {
            self.mem[i] = byte;
        }
    }

    fn emulate_cycle(&mut self) {
        // fetch
        self.opcode = (self.mem[self.pc as usize] as u16) << 8 | (self.mem[(self.pc as usize) + 1]) as u16;

        // match first nibble of opcode for instruction
        match self.opcode & 0xf000 {
            0x000 => self.instr_0(),
            0x100 => self.instr_1(),
            0x200 => self.instr_2(),
            0x300 => self.instr_3(),
            0x400 => self.instr_4(),
            0x500 => self.instr_5(),
            0x600 => self.instr_6(),
            0x700 => self.instr_7(),
            0x800 => self.instr_8(),
            0x900 => self.instr_9(),
            0xa00 => self.instr_a(),
            0xb00 => self.instr_b(),
            0xc00 => self.instr_c(),
            0xd00 => self.instr_d(),
            0xe00 => self.instr_e(),
            0xf00 => self.instr_f(),
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
        self.v[self.opcode_x()] += self.opcode_nn();
        self.pc += 2;
    }

    fn instr_8(&mut self) {
        match self.opcode & 0x000f {

        }

        self.pc += 2;
    }

    fn instr_9(&mut self) {
        // let x = ((self.opcode & 0x0f00) >> 8) as usize;
        // let y = ((self.opcode & 0x00f0) >> 4) as usize;

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
        let random_num: u8 = rand::thread_rng().gen();
        self.v[self.opcode_x()] = random_num & self.opcode_nn();

        self.pc += 2;
    }

    fn instr_d(&mut self) {

    }

    fn instr_e(&mut self) {

    }

    fn instr_f(&mut self) {

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
