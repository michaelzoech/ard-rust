
#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {

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