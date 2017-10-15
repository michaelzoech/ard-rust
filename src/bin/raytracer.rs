extern crate ard;

use std::rc::Rc;
use std::time::Instant;

use ard::camera::*;
use ard::color::*;
use ard::material::*;
use ard::math::*;
use ard::sampler::*;
use ard::shapes::*;

fn trace(ray: &Ray3, objects: &Vec<&Hitable>, depth: u32) -> Color {
    let have_hit = objects.into_iter()
        .filter_map(|o| o.intersect(&ray))
        .min_by(|a: &Intersection, b: &Intersection| a.t.partial_cmp(&b.t).unwrap());

    if let Some(intersection) = have_hit {
        let mut scattered = Ray3::default();
        let mut attenuation = Color::black();
        if depth < 10 && (*intersection.material).scatter(ray, &intersection, &mut attenuation, &mut scattered) {
            return trace(&scattered, objects, depth + 1) * attenuation;
        } else {
            return Color::black();
        }
    } else {
        let t = 0.5 * (ray.direction.y + 1.0);
        return Color::white() * (1.0-t) + Color {r: 0.5, g: 0.7, b: 1.0, a: 0.0} * t;
    }
}

fn main() {
    let width: u32 = 640;
    let height: u32 = 480;
    let pixel_size: f64 = 0.01;
    let mut render_buffer = ard::RenderBuffer::new(width, height);
    let sampler = Sampler::regular_sampler(4, 10.0);
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

    for y in 0..height {
        for x in 0..width {
            let mut color = Color::black();

            for &(sx, sy) in sampler.unit_square_samples.iter() {
                let dx = pixel_size * ((x as f64) - 0.5 * (width as f64) + (sx - 0.5));
                let dy = - pixel_size * ((y as f64) - 0.5 * (height as f64) + (sy - 0.5));
                let ray = camera.generate_ray(dx, dy);

                color += trace(&ray, &objects_vector, 0);
            }

            color /= sampler.unit_square_samples.len() as f64;

            render_buffer.set_pixel(x, y, color);
        }
    }

    let elapsed = start_time.elapsed().as_secs();

    println!("Image rendered in {0} seconds", elapsed);

    render_buffer.write_to_file("image.bmp").expect("Cannot write bitmap");    
}
