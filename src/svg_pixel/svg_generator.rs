use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;

pub struct SvgGenerator;

impl SvgGenerator {
    pub fn save(svg: String, target_path: &str) -> Result<&'static str, &'static str> {
        let f = File::create(target_path).expect("Failed to create file.");
        let mut writer = BufWriter::new(f);

        writer.write(&svg.into_bytes()).expect("Failed to write data on the file");

        Ok("Created SVG file successfully!")
    }
}
