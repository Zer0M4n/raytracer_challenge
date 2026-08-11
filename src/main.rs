mod camera;
mod canvas;
mod color;
mod computing;
mod math;
mod physics;
mod utils;

use crate::camera::camera::Camera;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::computing::*;
use crate::math::point::Point;
use crate::physics::intersect::Intersection;
use crate::physics::material::Point_Light;
use crate::physics::ray::Ray;
use crate::physics::sphere::Sphere;
// Import minifb windowing dependencies
use minifb::{Key, Window, WindowOptions};

fn main() {
    // let ray_origin = Point::new(0.0, 0.0, -5.0);
    // let wall_z = 10.0;
    // let wall_size = 7.0;

    // // You can keep it at 1000, but 800 is safer for performance during real-time rendering
    // let canvas_pixels = 800;

    // let pixel_size = wall_size / canvas_pixels as f64;
    // let half = wall_size / 2.0;

    // // 1. Initialize minifb window and the frame buffer
    // let mut window = Window::new(
    //     "Real-Time Ray Tracer",
    //     canvas_pixels,
    //     canvas_pixels,
    //     WindowOptions::default(),
    // )
    // .unwrap_or_else(|e| {
    //     panic!("{}", e);
    // });

    // // Limit window update rate to 60 FPS to prevent 100% CPU bottlenecking
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // // Minifb screen buffer (Format: 0x00RRGGBB)
    // let mut screen_buffer: Vec<u32> = vec![0; canvas_pixels * canvas_pixels];

    // // Maintain your original canvas struct in case you still want to export to PPM at the end
    // let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    // // Sphere setup
    // let mut shape = Sphere::new();
    // shape.material.color(Color::new(1.0, 0.2, 1.0));

    // // Light setup
    // let light_position = Point::new(-10.0, 10.0, -10.0);
    // let light_color = Color::new(1.0, 1.0, 1.0);
    // let light = Point_Light::new(light_position, light_color);

    // // 2. Control variables for real-time progressive rendering
    // let mut current_y = 0; // Tracks the horizontal line we are currently drawing
    // let mut rendering_done = false;

    // // 3. Main Window Event Loop
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     // Render exactly one row of pixels per frame to see the drawing progress in real-time
    //     if !rendering_done && current_y < canvas_pixels {
    //         let world_y = half - pixel_size * current_y as f64;

    //         for x in 0..canvas_pixels {
    //             let world_x = -half + pixel_size * x as f64;
    //             let position = Point::new(world_x, world_y, wall_z);

    //             let direction = (position - ray_origin).normalization();
    //             let ray = Ray::new(ray_origin, direction);

    //             let xs = shape.intersect(ray);

    //             if let Some(hit) = Intersection::hit(&xs) {
    //                 let point = ray.position(hit.t);
    //                 let normal = hit.object.normal_at(point);
    //                 let eye = -ray.direction;

    //                 let color = hit.object.material.lighting(&light, eye, normal);

    //                 // Write to your native canvas object
    //                 canvas.write_pixel(x, current_y, color);

    //                 // --- CONVERT COLOR TO MINIFB BITWISE FORMAT (0x00RRGGBB) ---
    //                 // Assuming Color uses floating points (0.0 to 1.0), multiply by 255.0 and clamp to avoid overflows
    //                 let r = (color.r * 255.0).clamp(0.0, 255.0) as u32;
    //                 let g = (color.g * 255.0).clamp(0.0, 255.0) as u32;
    //                 let b = (color.b * 255.0).clamp(0.0, 255.0) as u32;

    //                 // Combine channels into a single 32-bit integer using bitwise shifts
    //                 let pixel_u32 = (r << 16) | (g << 8) | b;

    //                 // Write pixel color directly to the screen frame buffer
    //                 screen_buffer[current_y * canvas_pixels + x] = pixel_u32;
    //             } else {
    //                 // Default background color (Black) if the ray doesn't intersect anything
    //                 screen_buffer[current_y * canvas_pixels + x] = 0x00000000;
    //             }
    //         }

    //         // Move down to the next row for the next frame iteration
    //         current_y += 1;

    //         if current_y >= canvas_pixels {
    //             rendering_done = true;
    //             println!("Rendering complete!");

    //             // Save PPM image output once the window finishes rendering
    //             canvas.canvas_to_ppm();
    //         }
    //     }

    //     // 4. Update window surface with the current progress of the screen buffer
    //     window
    //         .update_with_buffer(&screen_buffer, canvas_pixels, canvas_pixels)
    //         .unwrap();
    // }
}
