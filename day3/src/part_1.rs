use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process(input_file: &str) -> String {
    let file = File::open(input_file).expect("Input file should be in root directory");
    let reader = BufReader::new(file);

    let mut priorities: i32 = 0;
    for line in reader.lines() {
        let line = line.expect("valid line");
        let (first_part, second_part) = line.split_at(line.len() / 2);
        priorities = priorities + compare_parts(first_part, second_part);
    }
    String::from(format!("{priorities}"))
}

fn compare_parts(p1: &str, p2: &str) -> i32 {
    let mut value: i32 = 0;
    for c in p1.chars() {
        if p2.find(c).is_some() {
            if c.is_uppercase() {
                value = c as i32 - 38
            } else {
                value = c as i32 - 96
            }
        }
    }
    value
}
