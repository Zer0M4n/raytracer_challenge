use crate::utils::comparing_floating_number;
use std::ops::{Add,Sub,Mul};
use std::cmp::PartialEq;

#[derive(Debug,Clone, Copy)]
pub struct Color {
    red:f64,
    green:f64,
    blue:f64,
}

impl Color {

    pub fn new(red:f64 , green:f64 , blue:f64) -> Self {
        Self { red, green, blue }
    }

}

//Arithmetics Operators Overload

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red + rhs.red, 
            self.green + rhs.green, 
            self.blue + rhs.blue
        )       
    }
    
}
impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red - rhs.red, 
            self.green - rhs.green, 
            self.blue - rhs.blue
        )
    }
}
impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red * rhs.red, 
            self.green * rhs.green, 
            self.blue * rhs.blue
        )
    }
    
}
impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(
            self.red * rhs, 
            self.green * rhs, 
            self.blue * rhs
        )
    }
    
}

//Boolean Opertor overload
impl PartialEq for Color {
    
    fn eq(&self, other: &Self) -> bool {
        comparing_floating_number(self.red, other.red) &&
        comparing_floating_number(self.green, other.green) &&
        comparing_floating_number(self.blue, other.blue)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_does_stored_value() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }
    #[test]
    fn color_add_color() {
        let c = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c+c2, Color::new(1.6, 0.7, 1.0));

    }
    #[test]
    fn color_subtrac_color() {
        let c = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c-c2, Color::new(0.2, 0.5, 0.5 ));

    }
    #[test]
    fn color_multi_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
    
        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));

    }
    
    #[test]
    fn color_multi_color() {
        let c = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c * c2, Color::new(0.9, 0.2, 0.04));

    }
}