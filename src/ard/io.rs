use std::convert::AsRef;
use std::fs::File;
use std::io::BufWriter;
use std::io::Result;
use std::io::Write;
use std::path::Path;

pub struct OutputStream {
    writer: Box<Write>,
}

impl OutputStream {

    pub fn new<P: AsRef<Path>>(path: P) -> Result<OutputStream> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        Ok(OutputStream {
            writer: Box::new(writer)
        })
    }

    pub fn write(&mut self, array: &[u8]) -> Result<()> {
        self.writer.write_all(array)
    }

    pub fn write_u16_le(&mut self, value: u16) -> Result<()> {
        self.writer.write_all(&[(value&0xff) as u8, ((value>>8)&0xff) as u8])
    }

    pub fn write_u32_le(&mut self, value: u32) -> Result<()> {
        self.writer.write_all(&[(value&0xff) as u8, ((value>>8)&0xff) as u8, ((value>>16)&0xff) as u8, ((value>>24)&0xff) as u8])
    }
}