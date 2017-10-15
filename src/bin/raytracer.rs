extern crate ard;

use ard::camera::*;
use ard::color::*;
use ard::math::*;
use ard::sampler::*;
use ard::shapes::*;

fn main() {
    let width: u32 = 640;
    let height: u32 = 480;
    let pixel_size: f64 = 0.01;
    let mut render_buffer = ard::RenderBuffer::new(width, height);
    let sampler = Sampler::regular_sampler(4);
    let camera = OrthographicCamera::new(&Vector3::new(0.0, 0.0, 5.0), &Vector3::zero(), &Vector3::new(0.0, 1.0, 0.0));

    let objects = [
        Sphere {
            center: Vector3::new(1.0, 1.0, 0.0),
            radius: 0.5,
        },
        Sphere {
            center: Vector3::new(-1.0, 0.0, 0.0),
            radius: 0.5,
        },
        Sphere {
            center: Vector3::new(-0.0, 0.0, -2.0),
            radius: 1.5,
        },
    ];

    for y in 0..height {
        for x in 0..width {
            let mut color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0};

            for &(sx, sy) in sampler.unit_square_samples.iter() {
                let dx = pixel_size * ((x as f64) - 0.5 * (width as f64) + (sx - 0.5));
                let dy = - pixel_size * ((y as f64) - 0.5 * (height as f64) + (sy - 0.5));
                let ray = camera.generate_ray(dx, dy);

                let have_hit = objects.into_iter()
                    .filter_map(|o| o.intersect(&ray))
                    .min_by(|a: &Intersection, b: &Intersection| a.t.partial_cmp(&b.t).unwrap());

                if let Some(intersection) = have_hit {
                    color += Color {
                        r: 0.5 * (intersection.normal.x + 1.0),
                        g: 0.5 * (intersection.normal.y + 1.0),
                        b: 0.5 * (intersection.normal.z + 1.0),
                        a: 0.0,
                    };
                };
            }

            color /= sampler.unit_square_samples.len() as f64;

            render_buffer.set_pixel(x, y, color);
        }
    }

    render_buffer.write_to_file("image.bmp").expect("Cannot write bitmap");    
}
