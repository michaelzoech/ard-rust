use {TraceContext};
use color::{Color};
use math::{Ray3, Vector3};
use sampler::{Sampler};
use shapes::Intersection;

pub trait Material {

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
        let unit = 0.5 * (intersection.normal + Vector3::new(1.0, 1.0, 1.0));
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
    samples: Sampler,
    albedo: Color,
}

impl Material for Lambertian {

    fn scatter(&self, trace_context: &TraceContext, _: &Ray3, intersection: &Intersection, attenuation: &mut Color, scattered: &mut Ray3) -> bool {
        let w = intersection.normal;
        let v = (w.cross(&Vector3::new(0.0072, 1.0, 0.0034))).normalized();
        let u = v.cross(&w);

        let sample = self.samples.hemisphere_samples[trace_context.sample_index as usize];

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

    pub fn new(samples: &Sampler, albedo: &Color) -> Lambertian {
        Lambertian {
            samples: samples.clone(),
            albedo: *albedo,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Metal {
    samples: Sampler,
    albedo: Color,
}

impl Material for Metal {

    fn scatter(&self, _: &TraceContext, ray: &Ray3, intersection: &Intersection, attenuation: &mut Color, scattered: &mut Ray3) -> bool {
        let reflected = ray.direction.reflect(&intersection.normal);

        scattered.origin = intersection.point;
        scattered.direction = reflected;

        attenuation.r = self.albedo.r;
        attenuation.g = self.albedo.g;
        attenuation.b = self.albedo.b;

        scattered.direction.dot(&intersection.normal) > 0.0
    }
}

impl Metal {

    pub fn new(samples: &Sampler, albedo: &Color) -> Metal {
        Metal {
            samples: samples.clone(),
            albedo: *albedo,
        }
    }
}