use sdl2::keyboard::Keycode;

pub struct Keypad {
    key: [bool; 16]
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            key: [false; 16]
        }
    }

    // press down a key
    pub fn set(&mut self, key: Keycode) {
        println!{"Set key: {}", key};
        match key {
            Keycode::Num1 => {
                self.key[1] = true;
            }
            Keycode::Num2 => {
                self.key[2] = true;
            }
            Keycode::Num3 => {
                self.key[3] = true;
            }
            Keycode::Num4 => {
                self.key[12] = true;
            }
            Keycode::Q => {
                self.key[4] = true;
            }
            Keycode::W => {
                self.key[5] = true;
            }
            Keycode::E => {
                self.key[6] = true;
            }
            Keycode::R => {
                self.key[13] = true;
            }
            Keycode::A => {
                self.key[7] = true;
            }
            Keycode::S => {
                self.key[8] = true;
            }
            Keycode::D => {
                self.key[9] = true;
            }
            Keycode::F => {
                self.key[14] = true;
            }
            Keycode::Z => {
                self.key[10] = true;
            }
            Keycode::X => {
                self.key[0] = true;
            }
            Keycode::C => {
                self.key[11] = true;
            }
            Keycode::V => {
                self.key[15] = true;
            }
            _ => {}
        }
    }

    // unpress a key
    pub fn reset(&mut self, key: Keycode) {
        println!{"Reset key: {}", key};
        match key {
            Keycode::Num1 => {
                self.key[1] = false;
            }
            Keycode::Num2 => {
                self.key[2] = false;
            }
            Keycode::Num3 => {
                self.key[3] = false;
            }
            Keycode::Num4 => {
                self.key[12] = false;
            }
            Keycode::Q => {
                self.key[4] = false;
            }
            Keycode::W => {
                self.key[5] = false;
            }
            Keycode::E => {
                self.key[6] = false;
            }
            Keycode::R => {
                self.key[13] = false;
            }
            Keycode::A => {
                self.key[7] = false;
            }
            Keycode::S => {
                self.key[8] = false;
            }
            Keycode::D => {
                self.key[9] = false;
            }
            Keycode::F => {
                self.key[14] = false;
            }
            Keycode::Z => {
                self.key[10] = false;
            }
            Keycode::X => {
                self.key[0] = false;
            }
            Keycode::C => {
                self.key[11] = false;
            }
            Keycode::V => {
                self.key[15] = false;
            }
            _ => {}
        }
    }

    pub fn is_pressed(&self, i: usize) -> bool {
        self.key[i]
    }
}
