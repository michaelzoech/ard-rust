extern crate ard;

use ard::RenderBuffer;
use ard::color::Color;
use ard::sampler::Sampler;

fn main() {
    let size = 500;
    let dim = size as f64;
    let mut render_buffer = RenderBuffer::new(size, size);

    let sampler = Sampler::regular_sampler(8);

    for &(dx, dy) in sampler.unit_square_samples.iter() {
        let x = (dim * dx) as u32;
        let y = (dim * dy) as u32;
        render_buffer.set_pixel(x, y, Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0});
    }

    for &vec in sampler.hemisphere_samples.iter() {
        let x = (dim * (vec.x + 1.0) * 0.5).round() as u32;
        let y = (dim * (vec.y + 1.0) * 0.5).round() as u32;
        let z = (dim * (vec.z + 1.0) * 0.5).round() as u32;

        render_buffer.set_pixel(x, y, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0});
        render_buffer.set_pixel(x, z, Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0});
    }

    render_buffer.write_to_file("samplers.bmp").expect("Cannot write bitmap");
}