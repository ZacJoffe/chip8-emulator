pub struct Cpu {
    i: u16,
    v: [u8; 16],
    pc: u16,
    sp: u8,
    mem: [u8; 4096],
    sound_timer: u8,
    delay_timer: u8,
    opcode: u16
}

impl Cpu {
    fn new () -> Cpu {
        let mut cpu = Cpu {
            i: 0x200,
            v: [0; 16],
            pc: 0x200,
            sp: 0,
            mem: [0; 4096],
            sound_timer: 0,
            delay_timer: 0,
            opcode: 0
        }

        for i in 0..80 {
            cpu.mem[i] = fontset[i];
        }

        cpu
    }

    fn load_game(&mut self, game: Vec<u8>) {
        let data = vec![0; 0x200];
        // load data vector with program
        for byte in game {
            data.push(byte);
        }

        for (i, &byte) in data.iter().enumerate() {
            self.memory[i] = byte;
        }
    }

    fn emulate_cycle(&mut self) {
        // fetch
        self.opcode = self.mem[self.pc] << 8 | self.mem[self.pc + 1];

        // match first nibble of opcode for instruction
        match (self.opcode & 0xf000) {
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
            if (sound_timer == 1) {
                println!("BEEP!");
            }

            self.sound_timer -= 1;
        }
    }

    fn instr_0(&mut self) {

    }

    fn instr_1(&mut self) {

    }

    fn instr_2(&mut self) {

    }

    fn instr_3(&mut self) {

    }

    fn instr_4(&mut self) {

    }

    fn instr_5(&mut self) {

    }

    fn instr_6(&mut self) {

    }

    fn instr_7(&mut self) {

    }

    fn instr_8(&mut self) {

    }

    fn instr_9(&mut self) {

    }

    fn instr_a(&mut self) {

    }

    fn instr_b(&mut self) {

    }

    fn instr_c(&mut self) {

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
}

const fontset: [u8; 80] = [
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
