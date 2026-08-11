use crate::utils::comparing_floating_number;
pub struct Camera {
    hsize: u64,
    vsize: u64,
    field_of_view: f64,
}

impl Camera {
    pub fn new(hsize: u64, vsize: u64, field_of_view: f64) -> Self {
        Camera {
            hsize,
            vsize,
            field_of_view,
        }
    }
    fn pixel_size(&self) -> f64 {
        let half_view = (self.field_of_view / 2.0).tan();
        let aspect = self.hsize as f64 / self.vsize as f64;

        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        (half_width * 2.0) / self.hsize as f64
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

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
        assert!(comparing_floating_number(c.pixel_size(), 0.01))
    }
    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert!(comparing_floating_number(c.pixel_size(), 0.01))
    }
}
