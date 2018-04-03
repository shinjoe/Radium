use std::fs::File;
use std::io::{Write, BufWriter};

pub fn write_ppm(path: &str, ppm_data: &str) {
    let f = File::create(path).expect("Unable to create a file");
    let mut f = BufWriter::new(f);
    f.write_all(ppm_data.as_bytes()).expect("Unable to write data");
}