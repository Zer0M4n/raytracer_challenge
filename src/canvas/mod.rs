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
    fn read_pixel(&self, x: u64, y: u64) -> Color {
        self.pixels[y as usize][x as usize]
    }

    pub fn canvas_to_ppm(&self) -> std::io::Result<()> {
        let file = File::create("first_circle.ppm")?;
        let mut w = BufWriter::new(file);

        let width = self.width;
        let height = self.height;

        writeln!(w, "P3\n {width} {height} \n255\n")?;

        let mut j = 0;

        let mut ir = 0;
        let mut ig = 0;
        let mut ib = 0;

        while j < self.height {
            let mut i = 0;

            while i < self.width {
                let C = self.read_pixel(i, j);
                ir = (255.999 * C.red) as i64;
                ig = (255.999 * C.green) as i64;
                ib = (255.999 * C.blue) as i64;
                i += 1;

                writeln!(w, "{ir} {ig} {ib}")?;
            }
            j += 1;
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
