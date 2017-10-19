use {TraceContext};
use color::{Color};
use math::{Ray3, Vector3};
use sampler::{Sampler};
use shapes::Intersection;

pub trait Material {

    fn scatter(&self, trace_context: &TraceContext, ray: &Ray3, intersection: &Intersection, attenuation: &mut Color, scattered: &mut Ray3) -> bool;
}

#[derive(Clone, Debug)]
pub struct Lambertian {
    samples: Sampler,
    albedo: Color,
}

impl Material for Lambertian {

    fn scatter(&self, trace_context: &TraceContext, ray: &Ray3, intersection: &Intersection, attenuation: &mut Color, scattered: &mut Ray3) -> bool {
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

    fn scatter(&self, trace_context: &TraceContext, ray: &Ray3, intersection: &Intersection, attenuation: &mut Color, scattered: &mut Ray3) -> bool {
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