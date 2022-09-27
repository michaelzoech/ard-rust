extern crate ard;

use std::sync::Arc;
use std::time::Instant;

use ard::camera::*;
use ard::color::*;
use ard::material::*;
use ard::math::*;
use ard::sampler::*;
use ard::shapes::*;
use ard::trace::*;

fn main() {
    let config = RendererConfig {
        image_width: 1024,
        image_height: 768,
        pixel_size: 0.006,
        pixel_sampler: UnitSquareSampler::regular_sampler(8),
        max_trace_depth: 8,
        ambient_color: Color {
            r: 0.6,
            g: 0.8,
            b: 1.0,
            a: 1.0,
        },
        num_render_threads: None,
    };

    let mut renderer = Renderer::new(&config);

    //let camera = OrthographicCamera::new(&Vector3::new(0.0, 2.0, 4.5), &Vector3::new(0.0, 1.2, 0.0), &Vector3::new(0.0, 1.0, 0.0));
    let camera: Arc<dyn Camera> = Arc::new(PinholeCamera::new(
        &Vector3::new(0.0, 3.0, 4.5),
        &Vector3::new(0.0, 1.2, 0.0),
        &Vector3::new(0.0, 1.0, 0.0),
        4.0,
    ));

    let objects: Arc<Vec<Arc<dyn Hitable>>> = Arc::new(vec![
        Arc::new(Cube::new(
            Vector3::new(2.05, 1.0, 0.0),
            Vector3::new(2.0, 2.0, 2.0),
            Vector3::zero(),
            Arc::new(Metal::new(
                &UnitSphereSampler::random_sampler(64),
                &Color {
                    r: 0.8,
                    g: 0.8,
                    b: 0.8,
                    a: 1.0,
                },
                0.1,
            )),
        )),
        Arc::new(Sphere {
            center: Vector3::new(-2.05, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Metal::new(
                &UnitSphereSampler::random_sampler(64),
                &Color {
                    r: 0.8,
                    g: 0.6,
                    b: 0.2,
                    a: 1.0,
                },
                0.0,
            )),
        }),
        Arc::new(Sphere {
            center: Vector3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Lambertian::new(
                &HemiSphereSampler::jittered_sampler(8, 1.0),
                &Color {
                    r: 1.0,
                    g: 0.3,
                    b: 1.0,
                    a: 1.0,
                },
            )),
        }),
        Arc::new(Sphere {
            center: Vector3::new(2.0, 0.5, 1.5),
            radius: 0.5,
            material: Arc::new(Lambertian::new(
                &HemiSphereSampler::jittered_sampler(8, 10.0),
                &Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            )),
        }),
        Arc::new(Sphere {
            center: Vector3::new(0.0, -100.0, 0.0),
            radius: 100.0,
            //material: Rc::new(Lambertian::new(&HemiSphereSampler::jittered_sampler(8, 100.0), &Color { r: 0.8, g: 1.0, b: 0.0, a: 1.0 })),
            material: Arc::new(Metal::new(
                &UnitSphereSampler::random_sampler(64),
                &Color {
                    r: 0.8,
                    g: 0.8,
                    b: 0.8,
                    a: 1.0,
                },
                0.1,
            )),
        }),
    ]);

    let start_time = Instant::now();

    renderer.render(&camera, &objects);

    let elapsed = start_time.elapsed().as_secs();

    println!("Image rendered in {0} seconds", elapsed);

    renderer
        .write_to_file("image.bmp")
        .expect("Cannot write bitmap");
}
