mod ard;

fn main() {
    let mut render_buffer = ard::RenderBuffer::new(640, 480);

    for y in 0..480 {
        for x in 0..640 {
            let argb: u32 = (x&0xff) | ((y&0xff)<<8);
            render_buffer.set_pixel(x, y, argb);
        }
    }

    render_buffer.write_to_file("image.bmp").expect("Cannot write bitmap");    
}
