use std::f32::consts::{FRAC_PI_2};

use sdl2::rect::Point;
use sdl2::render::Canvas;

pub struct Boid {
    dir_x: f32,
    dir_y: f32,
    position: Point,
}

const TRIANGLE_SIZE: i32 = 20;
const SPEED: f32 = 10.0;

use sdl2::video::Window;
impl Boid {
    pub fn new(position: Point) -> Boid {
        Boid { dir_x: 0.1, dir_y: 0.2, position }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let top       = self.position.offset(0, -TRIANGLE_SIZE);
        let bot_left  = self.position.offset(-TRIANGLE_SIZE / 3, TRIANGLE_SIZE / 2);
        let bot_right = self.position.offset(TRIANGLE_SIZE / 3, TRIANGLE_SIZE / 2);

        // direction angle = t
        // tan t = y / x
        // rotation
        // | x cos t  -y sin t |
        // | x sin t   y cos t |

        let angle = (self.dir_y / self.dir_x).atan() + FRAC_PI_2;
        let s = angle.sin();
        let c = angle.cos();

        let ps: Vec<Point> = [top, bot_left, bot_right, top].iter().map(|p| {
            let x = (p.x() - self.position.x()) as f32;
            let y = (p.y() - self.position.y()) as f32;
            Point::new((x * c - y * s) as i32 + self.position.x(),
                       (x * s + y * c) as i32 + self.position.y())
        }).collect();

        canvas.draw_lines(&ps[..]).unwrap();
    }

    pub fn step(&mut self) {
        self.position = self.position.offset(
            (self.dir_x * SPEED) as i32,
            (self.dir_y * SPEED) as i32
        );
    }
}
