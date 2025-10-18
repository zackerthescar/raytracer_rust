#![allow(dead_code)]

use std::fs::File;
use std::io::Write;
use std::path::PathBuf; // For OS-agnostic filenames

use crate::pixel::Framebuffer;  // Access pixel from crate root

// Create a new file that ends in .ppm
pub fn new_ppm(name: &str) -> File {
    let mut path = PathBuf::from(name);
    path.set_extension("ppm");
    let file = match File::create(&path) {
        Err(e) => panic!("Could not create {}: {}", path.display(), e),
        Ok(file) => file,
    };
    return file;
}

// Writes
pub fn write_ppm_header(mut file: File, width: u32, height: u32) -> File {
    let header = format!("P3\n{} {}\n255\n", width, height);
    match file.write_all(header.as_bytes()) {
        Err(e) => panic!("Could not write to output file: {}", e),
        Ok(_) => file,
    }
}

pub fn write_buffer_data(mut file: File, fb: &Framebuffer) -> std::io::Result<File> {
    for pixel in fb.iter() {
        writeln!(file, "{}", pixel)?;
    }
    Ok(file)
}
