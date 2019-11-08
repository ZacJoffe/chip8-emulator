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
