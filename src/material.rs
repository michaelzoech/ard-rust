use crate::{TraceContext};
use crate::color::{Color};
use crate::math::{Ray3, Vector3};
use crate::sampler::{HemiSphereSampler, Sampler, UnitSphereSampler};
use crate::shapes::Intersection;

pub trait Material : Send + Sync {

    fn scatter(&self, trace_context: &TraceContext, ray: &Ray3, intersection: &Intersection, attenuation: &mut Color, scattered: &mut Ray3) -> bool;
}

#[derive(Clone, Debug)]
pub struct NullMaterial {
}

impl Material for NullMaterial {
    fn scatter(&self, _: &TraceContext, _: &Ray3, _: &Intersection, _: &mut Color, _: &mut Ray3) -> bool {
        false
    }
}

impl NullMaterial {
    pub fn new() -> NullMaterial {
        NullMaterial {
        }
    }
}

#[derive(Clone, Debug)]
pub struct NormalMaterial {
}

impl Material for NormalMaterial {
    fn scatter(&self, _: &TraceContext, _: &Ray3, intersection: &Intersection, attenuation: &mut Color, scattered: &mut Ray3) -> bool {
        scattered.origin = intersection.point + intersection.normal * 0.01;
        scattered.direction = intersection.normal;
        let unit = Vector3::new(intersection.normal.x.abs(), intersection.normal.y.abs(), intersection.normal.z.abs());
        attenuation.r = unit.x;
        attenuation.g = unit.y;
        attenuation.b = unit.z;
        true
    }
}

impl NormalMaterial {
    pub fn new() -> NormalMaterial {
        NormalMaterial {
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lambertian {
    samples: HemiSphereSampler,
    albedo: Color,
}

impl Material for Lambertian {

    fn scatter(&self, trace_context: &TraceContext, _: &Ray3, intersection: &Intersection, attenuation: &mut Color, scattered: &mut Ray3) -> bool {
        let w = intersection.normal;
        let v = (w.cross(&Vector3::new(0.0072, 1.0, 0.0034))).normalized();
        let u = v.cross(&w);

        let sample = self.samples.sample(trace_context.set_index, trace_context.sample_index);

        let target = u * sample.x + v * sample.y + w * sample.z;

        let normal = target.normalized();

        scattered.origin = intersection.point + normal * 0.01;
        scattered.direction = normal;

        attenuation.r = self.albedo.r;
        attenuation.g = self.albedo.g;
        attenuation.b = self.albedo.b;

        true
    }
}

impl Lambertian {

    pub fn new(samples: &HemiSphereSampler, albedo: &Color) -> Lambertian {
        Lambertian {
            samples: samples.clone(),
            albedo: *albedo,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Metal {
    samples: UnitSphereSampler,
    albedo: Color,
    fuzziness: f64,
}

impl Material for Metal {

    fn scatter(&self, trace_context: &TraceContext, ray: &Ray3, intersection: &Intersection, attenuation: &mut Color, scattered: &mut Ray3) -> bool {
        let sample = self.samples.sample(trace_context.set_index, trace_context.sample_index);
        let reflected = (ray.direction.reflect(&intersection.normal) + sample * self.fuzziness).normalized();

        scattered.origin = intersection.point;
        scattered.direction = reflected;

        attenuation.r = self.albedo.r;
        attenuation.g = self.albedo.g;
        attenuation.b = self.albedo.b;

        scattered.direction.dot(&intersection.normal) > 0.0
    }
}

impl Metal {

    pub fn new(samples: &UnitSphereSampler, albedo: &Color, fuzziness: f64,) -> Metal {
        Metal {
            samples: samples.clone(),
            albedo: *albedo,
            fuzziness: fuzziness,
        }
    }
}