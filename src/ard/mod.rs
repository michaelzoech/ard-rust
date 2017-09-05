use std;
use std::convert::AsRef;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

pub struct RenderBuffer {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl RenderBuffer {

    pub fn new(width: u32, height: u32) -> RenderBuffer {
        if width == 0 || height == 0 {
            panic!("Width or Height cannot be 0");
        }

        let mut pixels = Vec::new();
        pixels.resize((width * height * 4) as usize, 0);

        RenderBuffer {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn set_pixel(self: &mut RenderBuffer, x: u32, y: u32, argb: u32) {
        let start = ((y * self.width + x) * 4) as usize;
        self.pixels[start] = (argb & 0xff) as u8;
        self.pixels[start+1] = ((argb>>8) & 0xff) as u8;
        self.pixels[start+2] = ((argb>>16) & 0xff) as u8;
        self.pixels[start+3] = ((argb>>24) & 0xff) as u8;
    }

    pub fn write_to_file<P: AsRef<Path>>(self: &RenderBuffer, path: P) -> std::io::Result<()> {
        let mut out = OutputStream::new(path)?;

        let row_padding = ((4 - self.width % 4)) % 4;
        let header_size = 14 + 40;
        let image_size = (self.width * 3 + row_padding) * self.height;
        let file_size = header_size + image_size;

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
        out.write_u32_le(24)?;
        out.write_u32_le(0)?;
        out.write_u32_le(image_size)?;
        out.write_u32_le(0)?;
        out.write_u32_le(0)?;
        out.write_u32_le(0)?;
        out.write_u32_le(0)?;

        // pixels
        for y in 0..self.height {
            let mut start = (y * self.width * 4) as usize;
            for _ in 0..self.width {
                out.write(&self.pixels[start..start+3])?;
                start += 4;
            }
            if row_padding > 2 {
                out.write(&[0])?;
            }
            if row_padding > 1 {
                out.write(&[0])?;
            }
            if row_padding > 0 {
                out.write(&[0])?;
            }
        }

        Ok(())
    }
}

struct OutputStream {
    writer: Box<Write>,
}

impl OutputStream {

    fn new<P: AsRef<Path>>(path: P) -> std::io::Result<OutputStream> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        Ok(OutputStream {
            writer: Box::new(writer)
        })
    }

    fn write(self: &mut OutputStream, array: &[u8]) -> std::result::Result<(), std::io::Error> {
        self.writer.write_all(array)
    }

    fn write_u16_le(self: &mut OutputStream, value: u16) -> std::result::Result<(), std::io::Error> {
        self.writer.write_all(&[(value&0xff) as u8, ((value>>8)&0xff) as u8])
    }

    fn write_u32_le(self: &mut OutputStream, value: u32) -> std::result::Result<(), std::io::Error> {
        self.writer.write_all(&[(value&0xff) as u8, ((value>>8)&0xff) as u8, ((value>>16)&0xff) as u8, ((value>>24)&0xff) as u8])
    }
}