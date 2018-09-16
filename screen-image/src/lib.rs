extern crate image;

use image::Rgb;
use image::RgbImage;

#[derive(Copy, Clone)]
pub struct RGBColor(pub u8, pub u8, pub u8);

impl Into<Rgb<u8>> for RGBColor {
    fn into(self) -> Rgb<u8> {
        Rgb([self.0, self.1, self.2])
    }
}

type Point = (u32, u32);

#[derive(Clone)]
pub struct ColorPoint {
    point: Point,
    color: Rgb<u8>
}

impl ColorPoint {
    pub fn new((x, y): (u32, u32), (r, g, b): (u8, u8, u8)) -> Self {
        ColorPoint {
            point: (x, y),
            color: Rgb([r, g, b])
        }
    }
}

pub struct Screen {
    name: &'static str,
    buffer: RgbImage
}

impl Screen {
    pub fn new(title: &'static str, width: u32, height: u32) -> Screen {
        Screen {
            name: title,
            buffer: RgbImage::new(width, height)
        }
    }

    pub fn draw(&mut self, p: ColorPoint) -> Option<()> {
        self.buffer.put_pixel(p.point.0, p.point.1, p.color);
        Some(())
    }

    pub fn clear(&mut self, c: RGBColor) -> Option<()> {
        for pixel in self.buffer.pixels_mut() {
            *pixel = c.into()
        }
        Some(())
    }

    pub fn join(self) -> Result<(), std::io::Error> {
        self.buffer.save(String::from(self.name) + ".png")
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        self.buffer.save(String::from(self.name) + ".png").unwrap();
    }
}
