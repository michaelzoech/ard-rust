use std;
use rand::{self, Rng};

use {RenderBuffer, TraceContext};
use camera::Camera;
use color::Color;
use math::Ray3;
use sampler::Sampler;
use shapes::{Hitable, Intersection};

pub struct TraceConfig {
    pub image_width: u32,
    pub image_height: u32,
    pub pixel_size: f64,
    pub pixel_sampler: Sampler,
    pub max_trace_depth: u32,
}

pub struct Tracer {
    image_width: u32,
    image_height: u32,
    pixel_size: f64,
    pixel_sampler: Sampler,
    max_trace_depth: u32,
    image_buffer: RenderBuffer,
}

impl Tracer {

    pub fn new(config: &TraceConfig) -> Tracer {
        Tracer {
            image_width: config.image_width,
            image_height: config.image_height,
            pixel_size: config.pixel_size,
            pixel_sampler: config.pixel_sampler.clone(),
            max_trace_depth: config.max_trace_depth,
            image_buffer: RenderBuffer::new(config.image_width, config.image_height),
        }
    }

    pub fn write_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        self.image_buffer.write_to_file(path)
    }

    pub fn trace(&mut self, camera: &Camera, objects: &Vec<Box<Hitable>>) {
        let mut rng = rand::thread_rng();

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut color = Color::black();

                let num_samples = self.pixel_sampler.unit_square_samples.len() as u32;
                let sample_offset = rng.gen_range(0, num_samples);

                for (idx, &(sx, sy)) in self.pixel_sampler.unit_square_samples.iter().enumerate() {
                    let trace_context = TraceContext {
                        sample_index: (sample_offset + idx as u32) % num_samples,
                    };
                    let dx = self.pixel_size * ((x as f64) - 0.5 * (self.image_width as f64) + (sx - 0.5));
                    let dy = - self.pixel_size * ((y as f64) - 0.5 * (self.image_height as f64) + (sy - 0.5));
                    let ray = camera.generate_ray(dx, dy);

                    color += self.trace_ray(&trace_context, &ray, &objects, 0);
                }

                color /= self.pixel_sampler.unit_square_samples.len() as f64;

                self.image_buffer.set_pixel(x, y, color);
            }
        }
    }

    fn trace_ray(&self, trace_context: &TraceContext, ray: &Ray3, objects: &Vec<Box<Hitable>>, depth: u32) -> Color {
        let have_hit = objects.into_iter()
            .filter_map(|o| (*o).intersect(&ray))
            .min_by(|a: &Intersection, b: &Intersection| a.t.partial_cmp(&b.t).unwrap());

        if let Some(intersection) = have_hit {
            let mut scattered = Ray3::default();
            let mut attenuation = Color::black();
            if depth < self.max_trace_depth && (*intersection.material).scatter(trace_context, ray, &intersection, &mut attenuation, &mut scattered) {
                return self.trace_ray(trace_context, &scattered, objects, depth + 1) * attenuation;
            } else {
                return Color::black();
            }
        } else {
            let t = 0.5 * (ray.direction.y + 1.0);
            return Color::white() * (1.0-t) + Color {r: 0.5, g: 0.7, b: 1.0, a: 0.0} * t;
        }
    }
}
