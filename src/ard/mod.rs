#[macro_use]
mod macros;

pub mod camera;
pub mod color;
pub mod io;
pub mod math;
pub mod shapes;

use std;

use self::color::Color;
use self::io::OutputStream;

/// A 2-dimensional pixel buffer.
/// The x coordinate goes from 0 to width (exclusive), from left to right.
/// The y coordinate goes from 0 to height (exclusive), from top to bottom.
pub struct RenderBuffer {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl RenderBuffer {

    pub fn new(width: u32, height: u32) -> RenderBuffer {
        expect_neq!(width, 0);
        expect_neq!(height, 0);

        let mut pixels = Vec::new();
        pixels.resize((width * height) as usize, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 });

        RenderBuffer {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        expect_le!(x, self.width);
        expect_le!(y, self.height);

        let pos = (y * self.width + x) as usize;
        self.pixels[pos] = color;
    }

    pub fn write_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let mut out = OutputStream::new(path)?;

        let row_padding = ((4 - self.width % 4)) % 4;
        let header_size = 14 + 40;
        let image_size = (self.width * 3 + row_padding) * self.height;
        let file_size = header_size + image_size;

        let padding: Vec<u8> = std::iter::repeat(0).take(row_padding as usize).collect();

        // file header
        out.write(&[0x42, 0x4d])?;
        out.write_u32_le(file_size)?;
        out.write_u16_le(0)?;
        out.write_u16_le(0)?;
        out.write_u32_le(header_size)?;

        // info header
        out.write_u32_le(40)?;
        out.write_u32_le(self.width)?;
        out.write_u32_le(self.height)?;
        out.write_u16_le(1)?;
        out.write_u16_le(24)?;
        out.write_u32_le(0)?;
        out.write_u32_le(image_size)?;
        out.write_u32_le(0)?;
        out.write_u32_le(0)?;
        out.write_u32_le(0)?;
        out.write_u32_le(0)?;

        // Pixels are written in rows, starting from bottom-left.
        for y in 0..self.height {
            let mut index = ((self.height - y - 1) * self.width) as usize;
            for x in 0..self.width {
                let rgb = self.pixels[index].to_rgba32();
                out.write(&[(rgb&0xff) as u8, ((rgb>>8)&0xff) as u8, ((rgb>>16)&0xff) as u8])?;
                index += 1;
            }
            out.write(padding.as_slice())?;
        }

        Ok(())
    }
}