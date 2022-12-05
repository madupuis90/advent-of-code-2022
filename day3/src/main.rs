mod part_1;
mod part_2;

#[derive(Clone, Copy)]
enum Result {
    Lose = -1,
    Draw = 0,
    Win = 1,
}
impl Result {
    fn value(&self) -> i32 {
        match &self {
            Result::Win => 1,
            Result::Draw => 0,
            Result::Lose => -1,
        }
    }
}

fn main() {
    // println!("{}", part_1::process("input.txt"));
    // println!("{}", part_2::process("input.txt"));
    let a = Result::Win;
    let b = Result::Lose;
    let c = a as i32 + b as i32;
    let d: i32 = 0;
    let e = d as Result;
    println!("results are a={} and b={} and c={}", a as i32, b as i32, e);
}
