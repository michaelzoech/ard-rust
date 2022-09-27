//#![feature(integer_atomics)]

extern crate integer_atomics;
extern crate num_cpus;
extern crate rand;

#[macro_use]
mod macros;

pub mod camera;
pub mod color;
pub mod io;
pub mod material;
pub mod math;
pub mod sampler;
pub mod shapes;
pub mod trace;

use self::color::Color;
use self::io::OutputStream;

#[derive(Clone, Debug)]
pub struct TraceContext {
    pub set_index: usize,
    pub sample_index: usize,
}

/// A 2-dimensional pixel buffer.
/// The x coordinate goes from 0 to width (exclusive), from left to right.
/// The y coordinate goes from 0 to height (exclusive), from top to bottom.
#[derive(Clone, Debug)]
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
        pixels.resize(
            (width * height) as usize,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        );

        RenderBuffer {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        expect_lt!(x, self.width);
        expect_lt!(y, self.height);

        let pos = (y * self.width + x) as usize;
        self.pixels[pos] = color;
    }

    pub fn set_pixel_line(&mut self, y: u32, pixels: &Vec<Color>) {
        expect_lt!(y, self.height);
        expect_eq!(pixels.len(), self.width as usize);
        let pos = (y * self.width) as usize;
        self.pixels[pos..(pos + self.width as usize)].clone_from_slice(pixels);
    }

    pub fn write_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let mut out = OutputStream::new(path)?;

        let row_padding = (4 - self.width % 4) % 4;
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
            for _ in 0..self.width {
                let rgb = self.pixels[index].to_rgba32();
                out.write(&[
                    ((rgb >> 16) & 0xff) as u8,
                    ((rgb >> 8) & 0xff) as u8,
                    (rgb & 0xff) as u8,
                ])?;
                index += 1;
            }
            out.write(padding.as_slice())?;
        }

        Ok(())
    }
}
