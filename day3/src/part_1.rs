use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

pub fn process(input_file: &str) -> String {
    let file = File::open(input_file).expect("Input file should be in root directory");
    let reader = BufReader::new(file);
    String::from("bob")
}
