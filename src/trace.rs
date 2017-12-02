use num_cpus;
use std;
use std::option::Option;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use rand::{self, Rng};

use {RenderBuffer, TraceContext};
use camera::Camera;
use color::Color;
use math::{Ray3, Vector2};
use sampler::UnitSquareSampler;
use shapes::{Hitable, Intersection};

pub struct RendererConfig {
    pub image_width: u32,
    pub image_height: u32,
    pub pixel_size: f64,
    pub pixel_sampler: UnitSquareSampler,
    pub max_trace_depth: u32,
    pub ambient_color: Color,
    pub num_render_threads: Option<u32>,
}

#[derive(Clone)]
pub struct Renderer {
    image_width: u32,
    image_height: u32,
    pixel_size: f64,
    pixel_sampler: UnitSquareSampler,
    max_trace_depth: u32,
    ambient_color: Color,
    num_render_threads: u32,
    image_buffer: Arc<Mutex<RenderBuffer>>,
}

impl Renderer {

    pub fn new(config: &RendererConfig) -> Renderer {
        Renderer {
            image_width: config.image_width,
            image_height: config.image_height,
            pixel_size: config.pixel_size,
            pixel_sampler: config.pixel_sampler.clone(),
            max_trace_depth: config.max_trace_depth,
            ambient_color: config.ambient_color,
            num_render_threads: config.num_render_threads.unwrap_or_else(|| num_cpus::get() as u32),
            image_buffer: Arc::new(Mutex::new(RenderBuffer::new(config.image_width, config.image_height))),
        }
    }

    pub fn write_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        self.image_buffer.lock().unwrap().write_to_file(path)
    }

    pub fn render(&mut self, camera: &Arc<Camera>, objects: &Arc<Vec<Arc<Hitable>>>) {
        let mut handles = Vec::new();
        let next_line = Arc::new(AtomicU32::new(0));
        let tracer = Tracer {
            image_width: self.image_width,
            image_height: self.image_height,
            pixel_size: self.pixel_size,
            pixel_sampler: self.pixel_sampler.clone(),
            max_trace_depth: self.max_trace_depth,
            ambient_color: self.ambient_color,
            image_buffer: Arc::clone(&self.image_buffer),
        };

        for _ in 0..self.num_render_threads {
            let next_line = Arc::clone(&next_line);
            let camera = Arc::clone(camera);
            let objects = Arc::clone(objects);
            let tracer = tracer.clone();
            let handle = thread::spawn(move || {
                loop {
                    let y = next_line.fetch_add(1, Ordering::Relaxed);

                    if y >= tracer.image_height {
                        break;
                    }

                    tracer.trace_line(&camera, &objects, y);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

#[derive(Clone)]
struct Tracer {
    image_width: u32,
    image_height: u32,
    pixel_size: f64,
    pixel_sampler: UnitSquareSampler,
    max_trace_depth: u32,
    ambient_color: Color,
    image_buffer: Arc<Mutex<RenderBuffer>>,
}

impl Tracer {

    fn trace_line(&self, camera: &Arc<Camera>, objects: &Arc<Vec<Arc<Hitable>>>, y: u32) {
        let image_dim = Vector2::new(self.image_width as f64, self.image_height as f64);
        let half = Vector2::new(0.5, 0.5);
        let num_pixel_sets = self.pixel_sampler.samples.len();
        let mut rng = rand::thread_rng();
        let mut out = Vec::with_capacity(self.image_width as usize);

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

            out.push(color);
        }

        self.image_buffer.lock().unwrap().set_pixel_line(y, &out);
    }

    fn trace_ray(&self, trace_context: &TraceContext, ray: &Ray3, objects: &Vec<Arc<Hitable>>, depth: u32) -> Color {
        let have_hit = objects.into_iter()
            .filter_map(|o| (*o).intersect(&ray))
            .min_by(|a: &Intersection, b: &Intersection| a.t.partial_cmp(&b.t).unwrap());

        if let Some(intersection) = have_hit {
            let mut scattered = Ray3::default();
            let mut attenuation = Color::black();
            if (*intersection.material).scatter(trace_context, ray, &intersection, &mut attenuation, &mut scattered) && depth < self.max_trace_depth {
                return self.trace_ray(trace_context, &scattered, objects, depth + 1) * attenuation;
            } else {
                return attenuation;
            }
        } else {
            self.ambient_color
        }
    }
}
