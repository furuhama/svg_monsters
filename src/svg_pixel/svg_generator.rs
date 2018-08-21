#![allow(unused)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

pub struct SvgGenerator;

impl SvgGenerator {
    pub fn save(svg: String, target_path: &str) -> Result<&'static str, &'static str> {
        let f = File::create(target_path).expect("Failed to create file.");
        let mut writer = BufWriter::new(f);

        match writer.write(&svg.into_bytes()) {
            Err(_) => { Err("Failed to write data.") },
            Ok(_) => { Ok("Created SVG file successfully!") },
        }
    }
}
