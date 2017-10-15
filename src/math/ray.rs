use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray3 {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl CloseEq for Ray3 {

    fn close_eq(&self, rhs: &Ray3) -> bool {
        self.origin.close_eq(&rhs.origin) && self.direction.close_eq(&rhs.direction)
    }
}

impl Ray3 {

    pub fn new(origin: Vector3, direction: Vector3) -> Ray3 {
        Ray3 {
            origin: origin,
            direction,
        }
    }

    pub fn point_at(&self, t: f64) -> Vector3 {
        Vector3 {
            x: self.origin.x + self.direction.x * t,
            y: self.origin.y + self.direction.y * t,
            z: self.origin.z + self.direction.z * t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at_should_offset_by_parameter_multiplied_by_direction() {
        let ray = Ray3::new(Vector3{ x: 1.0, y: 0.0, z: 0.0}, Vector3 { x: 1.0, y: 1.0, z: 1.0 });
        assert_close!(Vector3::new(3.0, 2.0, 2.0), ray.point_at(2.0));
        assert_close!(Vector3::new(0.5, -0.5, -0.5), ray.point_at(-0.5));
    }
}