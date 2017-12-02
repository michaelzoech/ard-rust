use std::f64;
use std::mem;
use std::option::Option;
use std::sync::Arc;

use material::Material;
use math::{Matrix4, Ray3, Vector3};

#[derive(Clone)]
pub struct Intersection {
    pub ray: Ray3,
    pub t: f64,
    pub point: Vector3,
    pub normal: Vector3,
    pub material: Arc<Material>,
}

pub trait Hitable : Send + Sync {

    fn intersect(&self, ray: &Ray3) -> Option<Intersection>;
}

#[derive(Clone)]
pub struct Cube {
    center: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    material: Arc<Material>,
}

impl Hitable for Cube {

    fn intersect(&self, ray: &Ray3) -> Option<Intersection> {
        if let Some((t,normal)) = self.intersection_with_normal(ray) {
            Some(Intersection {
                ray: *ray,
                t: t,
                point: ray.point_at(t - 0.0001),
                normal: normal,
                material: self.material.clone(),
            })
        } else {
            None
        }
    }
}

impl Cube {

    pub fn new(center: Vector3, size: Vector3, rotation: Vector3, material: Arc<Material>) -> Cube {
        let m = Matrix4::rotation_x(rotation.x) * Matrix4::rotation_y(rotation.y) * Matrix4::rotation_z(rotation.z);
        Cube {
            center: center,
            u: m.transform_vector3(Vector3::new(size.x * 0.5, 0.0, 0.0)),
            v: m.transform_vector3(Vector3::new(0.0, size.y * 0.5, 0.0)),
            w: m.transform_vector3(Vector3::new(0.0, 0.0, size.z * 0.5)),
            material: material,
        }
    }

    pub fn intersection_with_normal(&self, ray: &Ray3) -> Option<(f64, Vector3)> {
        let mut tmin = f64::MIN;
        let mut vmin = Vector3::zero();
        let mut tmax = f64::MAX;
        let mut vmax = Vector3::zero();
        let p = self.center - ray.origin;
        let has_hit;

        {
            let mut hit = |a: Vector3, n: Vector3| {
                let ai = a.normalized();
                let len = a.length();
                let e = ai.dot(&p);
                let f = ai.dot(&ray.direction);
                if f.abs() > f64::EPSILON {
                    let mut t1 = (e + len) / f;
                    let mut t2 = (e - len) / f;
                    if t1 > t2 {
                        mem::swap(&mut t1, &mut t2);
                    }
                    if t1 > tmin { tmin = t1; vmin = n; }
                    if t2 < tmax { tmax = t2; vmax = n; }
                    if tmin > tmax || tmax < 0.0 {
                        return false;
                    } else {
                        return true;
                    }
                } else {
                    return true;
                }
            };

            has_hit = hit(self.u, self.v.cross(&self.w)) && hit(self.v, self.w.cross(&self.u)) && hit(self.w, self.u.cross(&self.v));
        }

        if has_hit {
            if tmin > 0.0 {
                vmin.normalize();
                Some((tmin, if ray.direction.dot(&vmin) > 0.0 { -vmin } else { vmin }))
            } else {
                vmax.normalize();
                Some((tmax, if ray.direction.dot(&vmax) > 0.0 { -vmax } else { vmax }))
            }
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub material: Arc<Material>,
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
            material: self.material.clone(),
        })
    }
}

#[derive(Clone)]
pub struct Plane {
    pub point: Vector3,
    pub normal: Vector3,
    pub material: Arc<Material>
}

impl Hitable for Plane {

    fn intersect(self: &Plane, ray: &Ray3) -> Option<Intersection> {
        let t = (self.point - ray.origin).dot(&self.normal) / ray.direction.dot(&self.normal);
        if t > 0.0001 {
            Some(Intersection {
                ray: *ray,
                t: t,
                point: ray.point_at(t),
                normal: self.normal,
                material: self.material.clone(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use material::NullMaterial;

    #[test]
    fn intersect() {
        let sphere = Sphere {
            center: Vector3 { x: 2.0, y: 0.0, z: 0.0 },
            radius: 1.0,
            material: Arc::new(NullMaterial::new()),
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