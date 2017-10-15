
use math::*;

pub trait Camera {

    fn generate_ray(&self, dx: f64, dy: f64) -> Ray3;
}

#[derive(Clone, Copy, Debug)]
pub struct OrthographicCamera {
    eye: Vector3,
    lookat: Vector3,
    direction: Vector3,
    uvw: (Vector3, Vector3, Vector3),
}

impl Camera for OrthographicCamera {

    fn generate_ray(&self, dx: f64, dy: f64) -> Ray3 {
        Ray3 {
            origin: self.eye + (self.uvw.0 * dx) + (self.uvw.1 * dy),
            direction: self.direction,
        }
    }
}

impl OrthographicCamera {

    pub fn new(eye: &Vector3, lookat: &Vector3, up: &Vector3) -> OrthographicCamera {
        OrthographicCamera {
            eye: *eye,
            lookat: *lookat,
            direction: (*lookat - *eye).normalized(),
            uvw: calculate_uvw(eye, lookat, up)
        }
    }
}

fn calculate_uvw(eye: &Vector3, lookat: &Vector3, up: &Vector3) -> (Vector3, Vector3, Vector3) {
    let w = (*eye - *lookat).normalized();
    let u = (up.cross(&w)).normalized();
    let v = w.cross(&u);
    (u, v, w)
}
