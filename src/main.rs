extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub mod boid;
use boid::Boid;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsys = sdl.video().unwrap();

    let window = video_subsys.window("boids", 640, 480).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut b = Boid::new(Point::new(50, 50));

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
        b.draw(&mut canvas);

        canvas.present();

        b.step();

        std::thread::sleep(std::time::Duration::new(0, 50_000_000));
    }
}
