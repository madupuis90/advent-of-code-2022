use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Copy, Clone)]
enum Pick {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Pick {
    fn from_str(character: &str) -> Pick {
        match character {
            "A" | "X" => Pick::Rock,
            "B" | "Y" => Pick::Paper,
            "C" | "Z" => Pick::Scissor,
            _ => panic!("Invalid Pick"),
        }
    }
}

pub fn process(input_file: &str) -> String {
    let file = File::open(input_file).expect("Input file should be in root directory");
    let reader = BufReader::new(file);

    let mut total_score: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("valid line");
        let vec = line.split(" ").collect::<Vec<&str>>();

        let opponent_pick = Pick::from_str(vec[0]) as i32;
        let my_pick = Pick::from_str(vec[1]) as i32;

        let match_score = (my_pick - opponent_pick + 3) % 3; // draw: 0, win: 1, lose: 2
        let match_score = (match_score + 1) * 3 % 9; // draw: 3, win: 6, lose: 0
        total_score = total_score + match_score + my_pick;
    }

    total_score.to_string()
}
