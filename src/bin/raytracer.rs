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
        pixel_sampler: UnitSquareSampler::regular_sampler(8),
        max_trace_depth: 10,
        ambient_color: Color { r: 0.6, g: 0.8, b: 1.0, a: 1.0 },
    };

    let mut tracer = Tracer::new(&trace_config);

    //let camera = OrthographicCamera::new(&Vector3::new(0.0, 2.0, 4.5), &Vector3::new(0.0, 1.2, 0.0), &Vector3::new(0.0, 1.0, 0.0));
    let camera = PinholeCamera::new(&Vector3::new(0.0, 2.0, 4.5), &Vector3::new(0.0, 1.2, 0.0), &Vector3::new(0.0, 1.0, 0.0), 4.0);

    let objects: Vec<Box<Hitable>> = vec![
        Box::new(Sphere {
            center: Vector3::new(2.0, 1.0, 0.0),
            radius: 1.0,
            material: Rc::new(Metal::new(&UnitSphereSampler::random_sampler(64), &Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 }, 0.1)),
        }),
        Box::new(Sphere {
            center: Vector3::new(-2.0, 1.0, 0.0),
            radius: 1.0,
            material: Rc::new(Metal::new(&UnitSphereSampler::random_sampler(64), &Color { r: 0.8, g: 0.6, b: 0.2, a: 1.0 }, 0.0)),
        }),
        Box::new(Sphere {
            center: Vector3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Rc::new(Lambertian::new(&HemiSphereSampler::jittered_sampler(8, 1.0), &Color { r: 1.0, g: 0.3, b: 1.0, a: 1.0 })),
        }),
        Box::new(Sphere {
            center: Vector3::new(0.0, -100.0, 0.0),
            radius: 100.0,
            //material: Rc::new(Lambertian::new(&HemiSphereSampler::jittered_sampler(8, 100.0), &Color { r: 0.8, g: 1.0, b: 0.0, a: 1.0 })),
            material: Rc::new(Metal::new(&UnitSphereSampler::random_sampler(64), &Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 }, 0.1)),
        }),
    ];

    let start_time = Instant::now();

    tracer.trace(&camera, &objects);

    let elapsed = start_time.elapsed().as_secs();

    println!("Image rendered in {0} seconds", elapsed);

    tracer.write_to_file("image.bmp").expect("Cannot write bitmap");
}
