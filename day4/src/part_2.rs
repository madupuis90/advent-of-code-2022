use std::fs::File;
use std::io::{BufRead, BufReader};

struct Assignment {
    left: (i32, i32),
    right: (i32, i32),
}
impl Assignment {
    fn new(left: (i32, i32), right: (i32, i32)) -> Self {
        Self { left, right }
    }
}
pub fn process(input_file: &str) -> String {
    let file = File::open(input_file).expect("Input file should be in root directory");
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;

    let vec: Vec<Assignment> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse(&line))
        .collect();

    for assignment in vec {
        let a = assignment.left.0;
        let b = assignment.left.1;
        let c = assignment.right.0;
        let d = assignment.right.1;
        // I found it easier to check if they don't overlap and reverse it
        if !(b < c || d < a) {
            sum = sum + 1;
        }
    }
    String::from(format!("{}", sum))
}

fn parse(line: &str) -> Assignment {
    let (pair_1, pair_2) = line.split_once(",").unwrap();
    let pair_1 = pair_1
        .split_once("-")
        .unwrap();
    let pair_2 = pair_2
        .split_once("-")
        .unwrap();
    Assignment::new(
        (
            pair_1
                .0
                .parse::<i32>()
                .unwrap(),
            pair_1
                .1
                .parse::<i32>()
                .unwrap(),
        ),
        (
            pair_2
                .0
                .parse::<i32>()
                .unwrap(),
            pair_2
                .1
                .parse::<i32>()
                .unwrap(),
        ),
    )
}
