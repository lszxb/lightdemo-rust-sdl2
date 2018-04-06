extern crate sdl2;
extern crate rand;

mod screen;

use sdl2::pixels::Color;
use screen::{ColorPoint, Screen};
use rand::Rng;

const TWO_PI: f32 = 6.28318530718;
const W: u32 = 512;
const H: u32 = 512;
const N: u32 = 256;
const MAX_STEP: u32 = 10;
const MAX_DISTANCE: f32 = 2.0;
const EPSILON: f32 = 1e-6;

pub fn main() {
    let screen = Screen::new("sdl2 test", W, H);
    screen.clear(Color::RGB(0, 0, 0)).unwrap();
    for y in 0..H {
        for x in 0..W {
            let c = (sample(x as f32 / W as f32, y as f32/ H as f32) * 255.0f32).min(255.0f32) as u8;
            screen.draw(ColorPoint::new((x, y), (c, c, c))).unwrap();
        }
    }
    screen.join().unwrap();
}

fn sample(x: f32, y: f32) -> f32 {
    let mut sum = 0.0f32;
    let mut rng = rand::thread_rng();
    for i in 0..N {
        let a = TWO_PI * (i as f32 + rng.gen::<f32>()) / N as f32;
        sum += trace(x, y, a.cos(), a.sin());
    }
    return sum / N as f32;
}

fn trace(ox: f32, oy: f32, dx: f32, dy: f32) -> f32 {
    let mut t = 0.0f32;
    let mut i = 0;
    while i < MAX_STEP && t < MAX_DISTANCE {
        let sd = circle_sdf(ox + dx * t, oy + dy * t, 0.5, 0.5, 0.1);
        if sd < EPSILON {
            return 2.0;
        }
        t += sd;
        i += 1;
    }
    0.0
}

fn circle_sdf(x: f32, y: f32, cx: f32, cy: f32, r: f32) ->f32 {
    let ux = x - cx;
    let uy = y - cy;
    (ux * ux + uy * uy).sqrt() - r
}
