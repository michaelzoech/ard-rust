use std;
use rand::{self, Rng};

use {RenderBuffer, TraceContext};
use camera::Camera;
use color::Color;
use math::{Ray3, Vector2};
use sampler::UnitSquareSampler;
use shapes::{Hitable, Intersection};

pub struct TraceConfig {
    pub image_width: u32,
    pub image_height: u32,
    pub pixel_size: f64,
    pub pixel_sampler: UnitSquareSampler,
    pub max_trace_depth: u32,
    pub ambient_color: Color,
}

pub struct Tracer {
    image_width: u32,
    image_height: u32,
    pixel_size: f64,
    pixel_sampler: UnitSquareSampler,
    max_trace_depth: u32,
    ambient_color: Color,
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
            ambient_color: config.ambient_color,
            image_buffer: RenderBuffer::new(config.image_width, config.image_height),
        }
    }

    pub fn write_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        self.image_buffer.write_to_file(path)
    }

    pub fn trace(&mut self, camera: &Camera, objects: &Vec<Box<Hitable>>) {
        let image_dim = Vector2::new(self.image_width as f64, self.image_height as f64);
        let half = Vector2::new(0.5, 0.5);
        let num_pixel_sets = self.pixel_sampler.samples.len();
        let mut rng = rand::thread_rng();

        for y in 0..self.image_height {
            let mut set_offset: usize = rng.gen();

            for x in 0..self.image_width {
                let mut color = Color::black();
                let sample_offset: usize = rng.gen();
                let pixel_corner = self.pixel_size * (Vector2 { x: x as f64, y: y as f64} - 0.5 * image_dim - half);

                set_offset += 1;

                let pixel_set_index = set_offset % num_pixel_sets;

                for (idx, &sample) in self.pixel_sampler.samples[pixel_set_index].iter().enumerate() {
                    let trace_context = TraceContext {
                        set_index: set_offset,
                        sample_index: sample_offset + idx,
                    };
                    let sampled_pixel_pos = pixel_corner + self.pixel_size * sample;
                    let ray = camera.generate_ray(sampled_pixel_pos.x, -sampled_pixel_pos.y);

                    color += self.trace_ray(&trace_context, &ray, &objects, 0);
                }

                color /= self.pixel_sampler.samples[pixel_set_index].len() as f64;

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
            self.ambient_color
        }
    }
}
