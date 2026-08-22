use crate::{
    color::Color,
    math::{
        point::{self, Point},
        vector::Vector,
    },
    physics::type_pattern::TypePattern,
};

#[derive(Debug, Clone, PartialEq, Copy)]

pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<TypePattern>,
}
#[derive(Debug, Clone, PartialEq, Copy)]

pub struct Point_Light {
    pub point: Point,
    intensity: Color,
}
impl Material {
    pub fn default() -> Self {
        let c = Color::new(1.0, 1.0, 1.0);
        Material {
            color: c,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }
    pub fn lighting(
        &self,
        light: &Point_Light,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        if let Some(pattern) = self.pattern {
            return pattern.at(point)
        } ;

        // combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity;

        // find the direction to the light source
        let lightv = (light.point - point).normalization();

        // compute the ambient contribution
        let ambient = effective_color * self.ambient;
        if in_shadow {
            return ambient;
        }
        let diffuse: Color;
        let specular: Color;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector.
        let light_dot_normal = lightv.dot_product(normalv);

        if light_dot_normal < 0.0 {
            diffuse = Color::new(0.0, 0.0, 0.0);
            specular = Color::new(0.0, 0.0, 0.0);
        } else {
            // compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector.
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot_product(eyev);

            if reflect_dot_eye <= 0.0 {
                specular = Color::new(0.0, 0.0, 0.0);
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
    pub fn color(&mut self, c: Color) {
        self.color = c;
    }

    pub fn ambient(&mut self, am: f64) {
        self.ambient = am;
    }

    pub fn diffuse(&mut self, diff: f64) {
        self.diffuse = diff;
    }

    pub fn specular(&mut self, sp: f64) {
        self.specular = sp;
    }

    pub fn shininess(&mut self, sh: f64) {
        self.shininess = sh;
    }
}

impl Point_Light {
    pub fn new(point: Point, intensity: Color) -> Self {
        Point_Light { point, intensity }
    }
}

#[cfg(test)]
mod tests {
    use crate::physics::type_pattern::Stripe_Pattern;

    use super::*;
    #[test]
    fn the_default_material() {
        let m = Material::default();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let point = Point::new(0.0, 0.0, 0.0);

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let material = Material::default();
        let light = Point_Light::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let result = material.lighting(&light, point, eyev, normalv, true);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1))
    }
    #[test]
    pub fn lighting_with_a_pattern_applied() {
        let mut m = Material::default();
        let stripe = Stripe_Pattern::new();
        m.pattern = Some(TypePattern::Stripe_Pattern(stripe));
        m.ambient(1.0);
        m.diffuse(0.0);
        m.specular(0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);

        let light = Point_Light::new(
            Point::new(0.0, 0.0, -10.0), 
            Color::new(1.0, 1.0, 1.0)
        );

        let c1 = m.lighting(&light, Point::new(0.9, 0.0, 0.0), eyev, normalv, false);
        let c2 = m.lighting(&light, Point::new(1.1, 0.0, 0.0), eyev, normalv, false);
        
        assert_eq!(c1, Color::new(1.0, 1.0, 1.0));
        assert_eq!(c2, Color::new(0.0, 0.0, 0.0));
    }
}
