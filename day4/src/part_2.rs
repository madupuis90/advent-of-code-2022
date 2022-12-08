use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process(input_file: &str) -> String {
    let file = File::open(input_file).expect("Input file should be in root directory");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("valid line");
    }
    String::from(format!("{}", 1))
}
