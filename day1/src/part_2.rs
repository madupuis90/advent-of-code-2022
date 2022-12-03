use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

pub fn process() -> String {
    let file = File::open("input.txt").expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut vec: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("valid line");
        if line.is_empty() {
            vec.push(sum);
            sum = 0;
            continue;
        }
        sum = sum + line.parse::<i32>().expect("line to be a integer");
    }

    vec.sort();
    vec.reverse();

    vec[0..3].iter().sum::<i32>().to_string()
}
