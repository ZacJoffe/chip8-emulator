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
        Cpu {
            i: 0x200,
            v: [0; 16],
            pc: 0x200,
            sp: 0,
            mem: [0; 4096],
            sound_timer: 0,
            delay_timer: 0,
            opcode: 0
        }
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
            0x000 => self.instr_0xxx(),
            0x100 => self.instr_1xxx(),
            0x200 => self.instr_2xxx(),
            0x300 => self.instr_3xxx(),
            0x400 => self.instr_4xxx(),
            0x500 => self.instr_5xxx(),
            0x600 => self.instr_6xxx(),
            0x700 => self.instr_7xxx(),
            0x800 => self.instr_8xxx(),
            0x900 => self.instr_9xxx(),
            0xa00 => self.instr_axxx(),
            0xb00 => self.instr_bxxx(),
            0xc00 => self.instr_cxxx(),
            0xd00 => self.instr_dxxx(),
            0xe00 => self.instr_exxx(),
            0xf00 => self.instr_fxxx(),
            _ => self.nop()
        }
    }

    // nop if instruction isn't valid
    fn nop(&self) {
        println!("Nop instruction");
    }
}

