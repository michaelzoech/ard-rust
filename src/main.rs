mod ard;

use ard::camera::*;
use ard::color::*;
use ard::math::*;
use ard::shapes::*;

fn main() {
    let width: u32 = 640;
    let height: u32 = 480;
    let pixel_size: f64 = 0.01;
    let mut render_buffer = ard::RenderBuffer::new(width, height);

    let objects = [
        Sphere {
            center: Vector3::new(1.0, 1.0, 0.0),
            radius: 0.5,
        },
        Sphere {
            center: Vector3::new(-1.0, 0.0, 0.0),
            radius: 0.5,
        }
    ];

    let camera = OrthographicCamera::new(&Vector3::new(0.0, 0.0, 5.0), &Vector3::zero(), &Vector3::new(0.0, 1.0, 0.0));

    for y in 0..height {
        for x in 0..width {
            let dx = pixel_size * ((x as f64) - 0.5 * (width as f64));
            let dy = - pixel_size * ((y as f64) - 0.5 * (height as f64));
            let ray = camera.generate_ray(dx, dy);

            let mut have_hit = objects.into_iter().filter_map(|o| o.intersect(&ray));

            let c = if have_hit.next() == None { 0.0 } else { 1.0 };

            render_buffer.set_pixel(x, y, Color { r: c, g: c, b: c, a: 1.0 });
        }
    }

    render_buffer.write_to_file("image.bmp").expect("Cannot write bitmap");    
}
