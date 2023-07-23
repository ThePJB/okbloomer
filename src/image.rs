use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use crate::vector::*;

pub struct ImageBuffer {
    w: usize,
    h: usize,
    data: Vec<u8>,
}

impl ImageBuffer {
    pub fn new(w: usize, h: usize) -> Self {
        ImageBuffer {
            w,
            h,
            data: vec![0; w*h*4],
        }
    }
    pub fn from_bytes(png_bytes: &[u8]) -> Self {
        // please explain
        let decoder = png::Decoder::new(png_bytes);
        let mut reader = decoder.read_info().unwrap();
        let mut data = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut data).unwrap();
        data.truncate(info.buffer_size());
        ImageBuffer {
            w: info.width as usize,
            h: info.height as usize,
            data,
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Vec4 {
        let idx = (y*self.w + x)*4;
        vec4(
            self.data[idx] as f32 / 255.0,
            self.data[idx+1] as f32 / 255.0,
            self.data[idx+2] as f32 / 255.0,
            self.data[idx+3] as f32 / 255.0,
        )
    }
    pub fn set(&mut self, x: usize, y: usize, colour: Vec4) {
        let idx = (y*self.w + x)*4;
        self.data[idx] = (colour.x * 255.0) as u8;
        self.data[idx + 1] = (colour.y * 255.0) as u8;
        self.data[idx + 2] = (colour.z * 255.0) as u8;
        self.data[idx + 3] = (colour.w * 255.0) as u8;
    }
    pub fn dump_to_file(&self, path: &str) {
        let path = Path::new(path);
        let file = File::create(path).unwrap();
        let ref mut buf_writer = BufWriter::new(file);

        let mut encoder = png::Encoder::new(buf_writer, self.w as u32, self.h as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        // encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8)); // maybe dont need lol
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000)
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&self.data).unwrap(); // Save
    }
}