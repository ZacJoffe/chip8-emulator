use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;

pub struct Graphics {
    gfx: [[u8; 64], 32], // represent graphics as a 2d array
    surface: Surface,
    draw_flag: bool
}

impl Graphics {
    pub fn new() -> Grapics {
        Graphics {
            gfx: [[0; 64]; 32],
            surface: Surface::new(
                640,
                320,
                PixelFormatEnum::RGB24
            ).unwrap(),
            draw_flag: true
        }
    }

    pub fn clear(&mut self) {
        self.gfx = [[0; 64]; 32];
        self.draw_flag = true;
    }

    pub fn update(&mut self, x: usize, y: usize, height: u8, i: u16, mem: [u8, 4096]) -> u8 {
        let mut pixel: u8;

        // return value
        let mut v15: u8 = 0;

        for yline in range(0, height) {
            pixel = mem[i + yline];

            for xline in range(0, 8) {
                if pixel & (0x80 >> xline) != 0 {
                    if self.gfx[(y + yline) % 32][(x + xline) % 64] == 1 {
                        v15 = 1;
                    }

                    self.gfx[(y + yline) % 32][(x + xline) % 64] ^= 1;
                }
            }
        }

        self.draw_flag = true;

        v15
    }
}
