#![feature(const_fn)]

#![allow(dead_code)]

extern crate rand;
#[cfg(feature = "output_sdl2")]
extern crate screen_sdl2 as output_screen;
#[cfg(feature = "output_image")]
extern crate screen_image as output_screen;

mod sdf;

use output_screen::{ColorPoint, Screen, RGBColor};
use rand::Rng;
use sdf::{SDF, Circle, Node};
use sdf::polygonal::PolygonalCapsule;

const TWO_PI: f32 = 6.28318530718;
const W: u32 = 512;
const H: u32 = 512;
const N: u32 = 128;
const MAX_STEP: u32 = 10;
const MAX_DISTANCE: f32 = 2.0;
const EPSILON: f32 = 1e-6;

pub fn main() {
    let mut screen = Screen::new("sdl2 test", W, H);
    screen.clear(RGBColor(0, 0, 0)).unwrap();
    for y in 0..H {
        for x in 0..W {
            let c = if "exactly" == std::env::args().last().unwrap() {
                (sample_exactly(x as f32 / W as f32, y as f32/ H as f32) * 255.0f32).min(255.0f32) as u8
            } else {
                (sample(&Node::new(x as f32 / W as f32, y as f32/ H as f32)) * 255.0f32).min(255.0f32) as u8
            };
            screen.draw(ColorPoint::new((x, y), (c, c, c))).unwrap();
        }
    }
    screen.join().unwrap();
}

fn sample_exactly(x: f32, y: f32) -> f32 {
    let d = ((x - 0.5).powi(2) + (y - 0.5).powi(2)).sqrt();
    const EMISSIVE: f32 = 2.0;
    if d < 0.1 {
        EMISSIVE
    } else {
        EMISSIVE *  (0.1 / d).asin() * 2.0 / TWO_PI
    }
}

fn sample(p: &Node) -> f32 {
    let mut sum = 0.0f32;
    let mut rng = rand::thread_rng();
    for i in 0..N {
        let a = TWO_PI * (i as f32 + rng.gen::<f32>()) / N as f32;
        sum += trace(p, &Node::new(a.cos(), a.sin()));
    }
    sum / N as f32

}

#[derive(Clone, Copy)]
struct Res {
    sd: f32,
    emissive: f32
}

impl std::ops::BitAnd for Res {
    type Output = Res;
    fn bitand(self, rhs: Res) -> Self::Output {
        if self.sd < rhs.sd { self } else { rhs }
    }
}

fn scene(p: &Node) -> Res {
    static C1: Circle = Circle::new(0.3, 0.3, 0.1);
    static C2: Circle = Circle::new(0.8, 0.8, 0.05);
    static P1: [Node; 3] = [Node::new(0.3, 0.8), Node::new(0.6, 0.7), Node::new(0.5, 0.5)];
    let t1 = PolygonalCapsule(&P1, 0.01);
    let r1 = Res {
        sd: C1.sdf(p),
        emissive: 2.0
    };
    let r2 = Res {
        sd: t1.sdf(p),
        emissive: 1.0
    };
    let r3 = Res {
        sd: C2.sdf(p),
        emissive: 2.0
    };
    r1 & r2 & r3
}

fn trace(o: &Node, d: &Node) -> f32 {
    let mut t = 0.0f32;
    let mut i = 0;
    while i < MAX_STEP && t < MAX_DISTANCE {
        let r = scene(&(o + d * t));
        if r.sd < EPSILON {
            return r.emissive;
        }
        t += r.sd;
        i += 1;
    }
    0.0
}
