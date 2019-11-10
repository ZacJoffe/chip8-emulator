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
}
