use crate::{
    canvas::Canvas,
    math::{matrix::Matrix, point::Point},
    physics::{ray::Ray, world::World},
};
use minifb::{Key, Window, WindowOptions};
#[derive(Clone)]
pub struct Camera {
    hsize: u64,
    vsize: u64,
    field_of_view: f64,

    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    pub transform: Matrix,
}

impl Camera {
    pub fn new(hsize: u64, vsize: u64, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let half_width: f64;
        let half_height: f64;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;
        let transform = Matrix::identity(4);
        Camera {
            hsize,
            vsize,
            field_of_view,
            half_width,
            half_height,
            pixel_size,
            transform,
        }
    }
    pub fn ray_for_pixel(&self, px: u64, py: u64) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = Point::new(world_x, world_y, -1.0);
        let origin = Point::new(0.0, 0.0, 0.0);

        let inverse = self.transform.inverse().unwrap();

        let pixel = inverse.clone() * pixel;
        let origin = inverse * origin;

        let direction = (pixel - origin).normalization();

        Ray::new(origin, direction)
    }
    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);

                let color = world.color_at(ray);

                if x % 100 == 0 && y % 100 == 0 {
                    println!("({}, {}) -> {:?}", x, y, color);
                }

                image.write_pixel(x, y, color);
            }
        }

        image
    }
    pub fn render_screen(&self, world: World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);

        let mut window = Window::new(
            "Ray Tracer - Real Time",
            self.hsize as usize,
            self.vsize as usize,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Limita la tasa de refresco para no saturar CPU
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let mut buffer: Vec<u32> = vec![0; (self.hsize * self.vsize) as usize];

        let mut current_y: u64 = 0;
        let mut rendering_done = false;

        while window.is_open() && !window.is_key_down(Key::Escape) {
            // Renderiza una fila por frame para ver el progreso en tiempo real
            if !rendering_done && current_y < self.vsize {
                for x in 0..self.hsize {
                    let ray = self.ray_for_pixel(x, current_y);
                    let color = world.color_at(ray);

                    canvas.write_pixel(x, current_y, color);

                    // Convierte el color (0.0-1.0) al formato 0x00RRGGBB de minifb
                    let r = (color.red * 255.0).clamp(0.0, 255.0) as u32;
                    let g = (color.green * 255.0).clamp(0.0, 255.0) as u32;
                    let b = (color.blue * 255.0).clamp(0.0, 255.0) as u32;
                    let pixel = (r << 16) | (g << 8) | b;

                    buffer[(current_y * self.hsize + x) as usize] = pixel;
                }

                current_y += 1;

                if current_y >= self.vsize {
                    rendering_done = true;
                    println!("Rendering complete!");
                }
            }

            window
                .update_with_buffer(&buffer, self.hsize as usize, self.vsize as usize)
                .unwrap();
        }

        canvas
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use minifb::Key::V;

    use crate::{
        color::Color, math::vector::Vector, physics::world::World, utils::view_transformation,utils::comparing_floating_number
    };

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let c = Camera::new(160, 120, PI / 2.0);
        assert_eq!(160, c.hsize);
        assert_eq!(120, c.vsize);
        assert_eq!(PI / 2.0, c.field_of_view)
    }
    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(comparing_floating_number(c.pixel_size, 0.01))
    }
    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert!(comparing_floating_number(c.pixel_size, 0.01))
    }
    #[test]
    fn contructing_a_ray_throught_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }
    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);

        c.transform = (Matrix::rotation_y(PI / 4.0) * Matrix::traslation(0.0, -2.0, 5.0)).unwrap();

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));

        assert_eq!(
            r.direction,
            Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }
    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        c.transform = view_transformation(from, to, up);
        let image = c.render(w);

        assert_eq!(image.read_pixel(5, 5), Color::new(0.38066, 0.47583, 0.2855))
    }
}
