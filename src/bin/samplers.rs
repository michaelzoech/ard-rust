extern crate ard;

use ard::RenderBuffer;
use ard::color::Color;
use ard::sampler::*;

fn render_unit_square_sampler(render_buffer: &mut RenderBuffer, offset_x: u32, offset_y: u32, dim: f64, sampler: &UnitSquareSampler) {
    for &sample in sampler.samples[0].iter() {
        let v = dim * sample;
        render_buffer.set_pixel(offset_x + v.x as u32, offset_y + v.y as u32, Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0});
    }
}

fn render_hemi_sphere_sampler(render_buffer: &mut RenderBuffer, offset_x: u32, offset_y: u32, dim: f64, sampler: &HemiSphereSampler) {
    for &vec in sampler.samples[0].iter() {
        let x = (dim * (vec.x + 1.0) * 0.5) as u32;
        let y = (dim * (vec.y + 1.0) * 0.5) as u32;
        let z = (dim * (vec.z + 1.0) * 0.5) as u32;

        render_buffer.set_pixel(offset_x + x, offset_y + y, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0});
        render_buffer.set_pixel(offset_x + x, offset_y + z, Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0});
    }
}


fn draw_line(render_buffer: &mut RenderBuffer, start_x: u32, start_y: u32, end_x: u32, end_y: u32) {
    let x0 = start_x as i32;
    let y0 = start_y as i32;
    let x1 = end_x as i32;
    let y1 = end_y as i32;
    let dx = x1 - x0;
    let dy = y1 - y0;

    let color = Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 };

    if dx > dy {
        let mut d = 2*dy - dx;
        let mut y = y0;

        for x in x0..x1 {
            render_buffer.set_pixel(x as u32, y as u32, color);
            if d > 0 {
                y += 1;
                d -= 2*dx;
            }
            d += 2*dy;
        }
    } else {
        let mut d = 2*dx - dy;
        let mut x = x0;

        for y in y0..y1 {
            render_buffer.set_pixel(x as u32, y as u32, color);
            if d > 0 {
                x += 1;
                d -= 2*dy;
            }
            d += 2*dx;
        }
    }
}

fn main() {
    let num_boxes = 4;
    let size = 520;
    let box_size = size / num_boxes;
    let box_dim = box_size as f64;
    let mut render_buffer = RenderBuffer::new(size, size);

    for i in 1..num_boxes {
        draw_line(&mut render_buffer, 0, box_size * i, size - 1, box_size * i);
        draw_line(&mut render_buffer, box_size * i, 0, box_size * i, size - 1);
    }

    for x in 0..num_boxes {
        let e = 10.0f64.powf(x as f64);
        render_unit_square_sampler(&mut render_buffer, box_size * x, box_size * 0, box_dim, &UnitSquareSampler::regular_sampler(8));
        render_hemi_sphere_sampler(&mut render_buffer, box_size * x, box_size * 0, box_dim, &HemiSphereSampler::regular_sampler(8, e));
        render_unit_square_sampler(&mut render_buffer, box_size * x, box_size * 1, box_dim, &UnitSquareSampler::jittered_sampler(8));
        render_hemi_sphere_sampler(&mut render_buffer, box_size * x, box_size * 1, box_dim, &HemiSphereSampler::jittered_sampler(8, e));
    }

    render_buffer.write_to_file("samplers.bmp").expect("Cannot write bitmap");
}