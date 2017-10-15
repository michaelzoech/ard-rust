use std::ops::Add;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::Mul;

#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl AddAssign for Color {

    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl DivAssign<f64> for Color {

    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.a /= rhs;
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl Color {

    pub fn black() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }

    pub fn white() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }

    pub fn to_rgba32(&self) -> u32 {
        let r = (clamp(self.r, 0.0, 1.0) * 255.0) as u32;
        let g = (clamp(self.g, 0.0, 1.0) * 255.0) as u32;
        let b = (clamp(self.b, 0.0, 1.0) * 255.0) as u32;
        let a = (clamp(self.a, 0.0, 1.0) * 255.0) as u32;
        r | (g<<8) | (b<<16) | (a<<24)
    }
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_rgba32_clamps_values_to_range() {
        let outside_range = Color { r: 1.1, g: 1.0, b: 1.0, a: -0.1 };
        assert_eq!(0x00ffffff, outside_range.to_rgba32());
    }

    #[test]
    fn to_rgba32_byte_order() {
        let mixed = Color { r: 0.1, g: 0.5, b: 1.0, a: 1.0 };
        assert_eq!(0xffff7f19, mixed.to_rgba32());
    }
}