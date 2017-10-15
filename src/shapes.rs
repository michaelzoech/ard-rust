use std::option::Option;

use math::{Ray3, Vector3};

#[derive(Clone, Copy)]
pub struct Intersection {
    pub ray: Ray3,
    pub t: f64,
    pub point: Vector3,
    pub normal: Vector3,
}

pub trait Hitable {

    fn intersect(&self, ray: &Ray3) -> Option<Intersection>;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

impl Hitable for Sphere {

    fn intersect(self: &Sphere, ray: &Ray3) -> Option<Intersection> {
        let v = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = (v * 2.0).dot(&ray.direction);
        let c = v.dot(&v) - self.radius * self.radius;
        let disc = b * b - 4.0 * a * c;

        if disc < 0.0 {
            return None;
        }

        let e = disc.sqrt();
        let denom = 2.0 * a;
        let t1 = (-b - e) / denom;
        let t2 = (-b + e) / denom;
        let t;

        if t1 > 0.0001 {
            t = t1;
        } else if t2 > 0.0001 {
            t = t2;
        } else {
            return None;
        }

        let point = ray.point_at(t);
        let normal = (point - self.center) / self.radius;

        Some(Intersection {
            ray: *ray,
            t: t,
            point: point,
            normal: normal,
        })
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