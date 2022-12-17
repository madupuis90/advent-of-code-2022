use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process(input_file: &str) -> String {
    let file = File::open(input_file).expect("Input file should be in root directory");
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;

    let vec: Vec<&str> = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();

    String::from(format!("{}", sum))
}
