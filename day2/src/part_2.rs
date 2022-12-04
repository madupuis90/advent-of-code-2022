use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Copy, Clone)]
enum Pick {
    Rock = 0,
    Paper = 1,
    Scissor = 2,
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
    fn from_i32(character: i32) -> Pick {
        match character {
            0 => Pick::Rock,
            1 => Pick::Paper,
            2 => Pick::Scissor,
            _ => panic!("Invalid number"),
        }
    }
    fn score(&self) -> i32 {
        match &self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scissor => 3,
        }
    }
}
#[derive(Copy, Clone)]
enum Result {
    Lose = -1,
    Draw = 0,
    Win = 1,
}
impl Result {
    fn from(character: &str) -> Result {
        match character {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Invalid Result"),
        }
    }
    fn score(&self) -> i32 {
        match &self {
            Result::Win => 1,
            Result::Draw => 0,
            Result::Lose => -1,
        }
    }
}
pub fn process(path: &str) -> String {
    let file = File::open(path).expect("Input file should be in root directory");
    let reader = BufReader::new(file);

    let mut total_score: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("valid line");
        let vec = line.split(" ").collect::<Vec<&str>>();
        let opponent_pick = Pick::from(vec[0]);
        let result = Result::from(vec[1]);
        let my_pick = figure_out_shape(&opponent_pick, &result);

        total_score = total_score + rock_paper_scissor(&opponent_pick, &my_pick) + &my_pick.score();
    }

    total_score.to_string()
}

fn figure_out_shape(opponent_pick: &Pick, result: &Result) -> Pick {
    let a = *opponent_pick as i32;
    let b = *result as i32;
    let r: i32 = (a + b + 3) % 3;
    println!("a={a}, b={b}, r={r}");
    Pick::from_i32(r)
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
