extern crate ard;

use std::rc::Rc;
use std::time::Instant;

use ard::camera::*;
use ard::color::*;
use ard::material::*;
use ard::math::*;
use ard::sampler::*;
use ard::shapes::*;
use ard::trace::*;

fn main() {

    let trace_config = TraceConfig {
        image_width: 640,
        image_height: 480,
        pixel_size: 0.01,
        pixel_sampler: Sampler::regular_sampler(4, 10.0),
        max_trace_depth: 10,
    };

    let mut tracer = Tracer::new(&trace_config);

    let camera = OrthographicCamera::new(&Vector3::new(0.0, 2.0, 4.5), &Vector3::new(0.0, 1.2, 0.0), &Vector3::new(0.0, 1.0, 0.0));

    let objects = [
        Sphere {
            center: Vector3::new(2.0, 1.0, 0.0),
            radius: 1.0,
            material: Rc::new(Metal::new(&Sampler::jittered_sampler(4, 100.0), &Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 })),
        },
        Sphere {
            center: Vector3::new(-2.0, 1.0, 0.0),
            radius: 1.0,
            material: Rc::new(Metal::new(&Sampler::jittered_sampler(4, 100.0), &Color { r: 0.8, g: 0.6, b: 0.2, a: 1.0 })),
        },
        Sphere {
            center: Vector3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Rc::new(Lambertian::new(&Sampler::jittered_sampler(4, 100.0), &Color { r: 0.8, g: 0.3, b: 0.3, a: 1.0 })),
        },
        Sphere {
            center: Vector3::new(0.0, -100.0, 0.0),
            radius: 100.0,
            material: Rc::new(Lambertian::new(&Sampler::jittered_sampler(4, 100.0), &Color { r: 0.8, g: 1.0, b: 0.0, a: 1.0 })),
        }
    ];

    let objects_vector = objects.iter().map(|s| s as &Hitable).collect();

    let start_time = Instant::now();

    tracer.trace(&camera, &objects_vector);

    let elapsed = start_time.elapsed().as_secs();

    println!("Image rendered in {0} seconds", elapsed);

    tracer.write_to_file("image.bmp").expect("Cannot write bitmap");
}
