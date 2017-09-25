use std::option::Option;

use ard::math::{Ray3, Vector3};

pub trait Hitable {

    fn intersect(&self, ray: &Ray3) -> Option<f64>;
}

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

impl Hitable for Sphere {

    fn intersect(self: &Sphere, ray: &Ray3) -> Option<f64> {
        let v = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = (v * 2.0).dot(&ray.direction);
        let c = v.dot(&v) - self.radius * self.radius;
        let disc = b * b - 4.0 * a * c;

        if disc < 0.0 {
            None
        } else {
            let e = disc.sqrt();
            let denom = 2.0 * a;
            let t = (-b - e) / denom;
            let t2 = (-b + e) / denom;
            if t > 0.0001 {
                Some(t)
            } else if t2 > 0.0001 {
                Some(t2)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
        let sphere = Sphere {
            center: Vector3 { x: 2.0, y: 0.0, z: 0.0 },
            radius: 1.0,
        };
        let ray = Ray3 {
            origin: Vector3::zero(),
            direction: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
        };
        let hit =  sphere.intersect(&ray);

        if let None = hit {
            assert!(false);
        }
    }
}