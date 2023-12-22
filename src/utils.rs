use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn readfile(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut out: Vec<String> = Vec::new();

    for (_ , result) in reader.lines().enumerate() {
    out.insert(0, result.unwrap()) 
    }
    return out;
}