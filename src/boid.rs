use std::f32::consts::{FRAC_PI_2};

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::vector2::Vector2;

const NEIGHBOURS_RADIUS: i32 = 50;

#[derive(PartialEq, Clone)]
pub struct Boid {
    dir: Vector2,
    pos: Point,
}

const TRIANGLE_SIZE: i32 = 20;
const SPEED: f32 = 15.0;

impl Boid {
    pub fn new(x: i32, y: i32, dir_x: f32, dir_y: f32) -> Boid {
        let d = Vector2::new(dir_x, dir_y);
        Boid { dir: d / d.norm(), pos: Point::new(x, y) }
    }

    pub fn step(&mut self, boids: &Vec<Boid>, width: i32, height: i32) {
        let ns = self.neighbours(boids);

        let ns_len = if ns.len() != 0 { ns.len() } else { 1 };

        let center = ns.iter()
            .fold(Point::new(0, 0), |acc, x| acc + x.pos) / ns_len as i32;
        let mut center_dir = Vector2::from_point(center - self.pos);
        center_dir.normalize();

        let mut align_dir = ns.iter()
            .fold(Vector2::new(0.0, 0.0), |acc, x| acc + x.dir) / ns_len as f32;
        align_dir.normalize();

        let sep_dir = Vector2::from_point(ns.iter()
            .fold(Point::new(0, 0), |acc, x| acc + (x.pos - self.pos)) / ns_len as i32) * -1.0;


        if ns.len() != 0 {
            self.dir = (self.dir + align_dir + center_dir + sep_dir) / 4.0;
        }
        self.dir.normalize();

        self.pos = self.pos.offset(
            (self.dir.x * SPEED as f32) as i32,
            (self.dir.y * SPEED as f32) as i32
        );
        self.pos.x %= width;
        self.pos.y %= height;
    }

    fn neighbours<'a>(&self, boids: &'a Vec<Boid>) -> Vec<&'a Boid> {
        boids.iter().filter(|n| self.dist(n) <= NEIGHBOURS_RADIUS && *n != self).collect()
    }

    fn dist(&self, other: &Boid) -> i32 {
        let p = self.pos - other.pos;
        ((p.x * p.x + p.y * p.y) as f32).sqrt() as i32
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let top       = self.pos.offset(0, -TRIANGLE_SIZE);
        let bot_left  = self.pos.offset(-TRIANGLE_SIZE / 3, TRIANGLE_SIZE / 2);
        let bot_right = self.pos.offset(TRIANGLE_SIZE / 3, TRIANGLE_SIZE / 2);

        // direction angle = t
        // tan t = y / x
        // rotation
        // | x cos t  -y sin t |
        // | x sin t   y cos t |

        let angle = (self.dir.y / self.dir.x).atan() + FRAC_PI_2;
        let s = angle.sin();
        let c = angle.cos();

        let ps: Vec<Point> = [top, bot_left, bot_right, top].iter().map(|p| {
            let x = (p.x() - self.pos.x()) as f32;
            let y = (p.y() - self.pos.y()) as f32;
            Point::new((x * c - y * s) as i32 + self.pos.x(),
                       (x * s + y * c) as i32 + self.pos.y())
        }).collect();

        canvas.draw_lines(&ps[..]).unwrap();
    }
}
