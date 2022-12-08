use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process(input_file: &str) -> String {
    let file = File::open(input_file).expect("Input file should be in root directory");
    let reader = BufReader::new(file);

    let mut priorities: i32 = 0;
    let mut group: Vec<String> = vec![String::new(); 3];

    let mut idx: usize = 0;
    for line in reader.lines() {
        let line = line.expect("valid line");
        group[idx] = line;

        if idx == 2 {
            priorities = priorities + find_badges(&group);
        }
        idx = (idx + 1) % 3;
    }

    String::from(format!("{}", priorities))
}

fn find_badges(group: &Vec<String>) -> i32 {
    let group_0: HashSet<char> = HashSet::from_iter(group[0].chars());
    let group_1: HashSet<char> = HashSet::from_iter(group[1].chars());
    let group_2: HashSet<char> = HashSet::from_iter(group[2].chars());
    let temp: HashSet<char> = group_1.intersection(&group_2).copied().collect();
    let mut value = 0;
    for c in group_0.intersection(&temp) {
        if c.is_uppercase() {
            value = c.clone() as i32 - 38;
        } else {
            value = c.clone() as i32 - 96;
        }
    }
    value
}
