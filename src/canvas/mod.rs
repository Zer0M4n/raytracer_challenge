use crate::color::Color;

use std::fs::File;
use std::io::{BufWriter, Write};

const BLACK: Color = Color::new(0.0, 0.0, 0.0);

#[derive(Debug, Clone)]
pub struct Canvas {
    width: u64,
    height: u64,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: u64, height: u64) -> Self {
        let pixels = vec![vec![BLACK; width as usize]; height as usize];

        Self {
            width,
            height,
            pixels,
        }
    }
    pub fn write_pixel(&mut self, x: u64, y: u64, color: Color) {
        self.pixels[y as usize][x as usize] = color;
    }
    pub fn read_pixel(&self, x: u64, y: u64) -> Color {
        self.pixels[y as usize][x as usize]
    }

    pub fn canvas_to_ppm(&self) -> std::io::Result<()> {
        let file = File::create("img/first_image_with_pattern.ppm")?;
        let mut w = BufWriter::new(file);

        writeln!(w, "P3")?;
        writeln!(w, "{} {}", self.width, self.height)?;
        writeln!(w, "255")?;

        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.read_pixel(x, y);

                let red = (255.999 * c.red.clamp(0.0, 1.0)) as i64;
                let green = (255.999 * c.green.clamp(0.0, 1.0)) as i64;
                let blue = (255.999 * c.blue.clamp(0.0, 1.0)) as i64;

                writeln!(w, "{} {} {}", red, green, blue)?;
            }
        }

        Ok(())
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn canvas_stored_values() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        for row in canvas.pixels {
            for pixel in row {
                assert_eq!(pixel, BLACK);
            }
        }
    }

    #[test]
    fn write_pixel_in_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let color = Color::new(1.0, 1.0, 1.0);

        canvas.write_pixel(4, 5, color);

        assert_eq!(canvas.read_pixel(4, 5), color);
        assert_eq!(canvas.read_pixel(4, 6), BLACK);
    }
}
