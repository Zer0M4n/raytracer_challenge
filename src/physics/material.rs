use crate::color::Color;

pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
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
        }
    }
    pub fn color(mut self, c: Color) {
        self.color = c;
    }

    pub fn ambient(mut self, am: f64) {
        self.ambient = am;
    }

    pub fn diffuse(mut self, diff: f64) {
        self.diffuse = diff;
    }

    pub fn specular(mut self, sp: f64) {
        self.specular = sp;
    }

    pub fn shininess(mut self, sh: f64) {
        self.shininess = sh;
    }
}
