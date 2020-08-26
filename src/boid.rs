use std::f64::consts::{FRAC_PI_2};

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::vector2::Vector2;

const NEIGHBOURS_RADIUS: f64 = 30.0;

#[derive(PartialEq, Clone)]
pub struct Boid {
    dir: Vector2,
    pos: Vector2,
}

const TRIANGLE_SIZE: f64 = 8.0;
const SPEED: f64 = 1.0;

impl Boid {
    pub fn new(x: f64, y: f64, dir_x: f64, dir_y: f64) -> Boid {
        let d = Vector2::new(dir_x, dir_y);
        Boid { dir: d / d.norm(), pos: Vector2::new(x, y) }
    }

    pub fn step(&mut self, boids: &Vec<Boid>, width: i32, height: i32) {
        let ns = self.neighbours(boids);

        if ns.len() == 0 {
            self.update_pos(width, height);
            return ;
        }

        let center = ns.iter()
            .fold(Vector2::new(0.0, 0.0), |acc, x| acc + x.pos) / ns.len() as f64;
        let mut center_dir = center - self.pos;

        let mut align_dir = ns.iter()
            .fold(Vector2::new(0.0, 0.0), |acc, x| acc + x.dir) / ns.len() as f64;

        let mut sep_dir = ns.iter()
            .fold(Vector2::new(0.0, 0.0), |acc, x| {
                acc + ((self.pos - x.pos) / (self.dist(x) * self.dist(x)))
            }) / ns.len() as f64;

        let max_speed = 4.0;
        align_dir.set_mag(max_speed);
        center_dir.set_mag(max_speed);
        sep_dir.set_mag(max_speed);

        let mut alignment_force = align_dir - self.dir;
        let mut center_force = center_dir - self.dir;
        // let mut sep_force = sep_dir - self.dir;

        let max_force = 1.0;
        alignment_force.limit(max_force);
        center_force.limit(max_force);
        sep_dir.limit(max_force);

        self.update_pos(width, height);
        let acceleration = alignment_force + center_force + sep_dir;
        self.dir += acceleration;
        self.dir.limit(max_speed);
    }

    fn update_pos(&mut self, width: i32, height: i32) {
        self.pos += self.dir * SPEED;
        self.pos.x = self.pos.x.rem_euclid(width as f64);
        self.pos.y = self.pos.y.rem_euclid(height as f64);
    }

    fn neighbours<'a>(&self, boids: &'a Vec<Boid>) -> Vec<&'a Boid> {
        boids.iter().filter(|n| self.dist(n) <= NEIGHBOURS_RADIUS && *n != self).collect()
    }

    fn dist(&self, other: &Boid) -> f64 {
        let p = self.pos - other.pos;
        (p.x * p.x + p.y * p.y).sqrt()
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let top       = self.pos + Vector2::new(0.0, -TRIANGLE_SIZE);
        let bot_left  = self.pos + Vector2::new(-TRIANGLE_SIZE / 3.0, TRIANGLE_SIZE / 2.0);
        let bot_right = self.pos + Vector2::new(TRIANGLE_SIZE / 3.0, TRIANGLE_SIZE / 2.0);

        // direction angle = t
        // tan t = y / x
        // rotation
        // | x cos t  -y sin t |
        // | x sin t   y cos t |

        let angle = (self.dir.y / self.dir.x).atan() + FRAC_PI_2;
        let s = angle.sin();
        let c = angle.cos();

        let ps: Vec<Point> = [top, bot_left, bot_right, top].iter().map(|p| {
            let x = (p.x - self.pos.x) as f64;
            let y = (p.y - self.pos.y) as f64;
            Point::new(((x * c - y * s) + self.pos.x) as i32,
                       ((x * s + y * c) + self.pos.y) as i32)
        }).collect();

        canvas.draw_lines(&ps[..]).unwrap();
    }
}
