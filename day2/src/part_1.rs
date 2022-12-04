use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

enum Pick {
    Rock,
    Paper,
    Scissor,
}

impl Pick {
    fn from(character: &str) -> Pick {
        match character {
            "A" | "X" => Pick::Rock,
            "B" | "Y" => Pick::Paper,
            "C" | "Z" => Pick::Scissor,
            _ => panic!("Invalid Pick"),
        }
    }
    fn value(&self) -> i32 {
        match &self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scissor => 3,
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
        let opponent_pick = Pick::from(vec[0]);
        let my_pick = Pick::from(vec[1]);

        total_score = total_score + rock_paper_scissor(&opponent_pick, &my_pick) + &my_pick.value();
    }

    total_score.to_string()
}

fn rock_paper_scissor(opponent_pick: &Pick, my_pick: &Pick) -> i32 {
    const WIN: i32 = 6;
    const DRAW: i32 = 3;
    const LOSE: i32 = 0;
    match opponent_pick {
        Pick::Rock => match my_pick {
            Pick::Paper => WIN,
            Pick::Scissor => LOSE,
            Pick::Rock => DRAW,
        },
        Pick::Paper => match my_pick {
            Pick::Scissor => WIN,
            Pick::Rock => LOSE,
            Pick::Paper => DRAW,
        },
        Pick::Scissor => match my_pick {
            Pick::Rock => WIN,
            Pick::Paper => LOSE,
            Pick::Scissor => DRAW,
        },
    }
}
