extern crate sdl2;

mod screen;

use sdl2::pixels::Color;
use screen::{ColorPoint, Screen};

pub fn main() {
    let screen = Screen::new("sdl2 test", 800, 600);
    screen.clear(Color::RGB(0, 255, 255)).unwrap();
    for i in 100..300 {
        screen.draw(ColorPoint::new((i, i), (0, 0, 0))).unwrap();
    }
    screen.join().unwrap();
}
