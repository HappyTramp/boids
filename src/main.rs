extern crate sdl2;
extern crate rand;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use rand::Rng;

pub mod vector2;
pub mod boid;
use boid::Boid;

const HEIGHT:            i32   = 480;
const WIDTH:             i32   = 640;
const BOIDS_NUM:         usize = 100;
// const NEIGHBOURS_RADIUS: i32   = ((HEIGHT + WIDTH) / 2) / 10;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsys = sdl.video().unwrap();

    let window = video_subsys.window("boids", WIDTH as u32, HEIGHT as u32).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut boids: Vec<Boid> = Vec::with_capacity(BOIDS_NUM);

    let mut rng = rand::thread_rng();
    for _ in 0..BOIDS_NUM {
        boids.push(Boid::new(
            rng.gen_range(0.0, WIDTH as f64),
            rng.gen_range(0.0, HEIGHT as f64),
            rng.gen_range(0.1, 1.0),
            rng.gen_range(0.1, 1.0),
        ));
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Q), .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                    => break 'main,
                _ => {},
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        for b in boids.iter() {
            b.draw(&mut canvas);
        }

        canvas.present();

        let prev_boids = boids.clone();

        for b in boids.iter_mut() {
            b.step(&prev_boids, WIDTH, HEIGHT);
        }

        std::thread::sleep(std::time::Duration::new(0, 10_000_000));
    }
}
