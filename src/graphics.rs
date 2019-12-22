use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Graphics<'a> {
    gfx: [[u8; 64]; 32], // represent graphics as a 2d array
    surface: Surface<'a>,
    draw_flag: bool
}

impl<'a> Graphics<'a> {
    pub fn new() -> Graphics<'a> {
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

    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        if self.draw_flag {
            canvas.clear();
            for x in 0..64 {
                for y in 0..32 {
                    /*
                    if self.gfx[y][x] == 0 {
                        // black
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                    } else {
                        // white
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }

                    canvas.fill_rect(Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10));
                    */

                    if self.gfx[y][x] == 1 {
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                        canvas.fill_rect(Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10)).unwrap();
                    } else {
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                        canvas.fill_rect(Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10)).unwrap();
                    }
                }
            }

            canvas.present();
            self.draw_flag = false;
        }
    }

    pub fn update(&mut self, x: usize, y: usize, height: u8, i: u16, mem: [u8; 4096]) -> u8 {
        let mut pixel: u8;

        // return value
        let mut v15: u8 = 0;

        for yline in 0..height as usize {
            pixel = mem[i as usize + yline];

            for xline in 0..8 as usize {
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
