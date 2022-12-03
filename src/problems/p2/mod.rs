use std::{fs, ops::Sub, path::Path};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

fn read_input(sample: bool) -> String {
    let file_path = Path::new(file!());
    let path = file_path.with_file_name(if sample {
        "sample_input.txt"
    } else {
        "input.txt"
    });
    let contents = fs::read_to_string(path).unwrap();
    return contents;
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl Sub for RPS {
    type Output = i32;
    fn sub(self, rhs: Self) -> Self::Output {
        return self as i32 - rhs as i32;
    }
}

impl RPS {
    fn from_yours(result: &str) -> RPS {
        match result {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => panic!("Invalid yours result, {}", result),
        }
    }

    fn from_theirs(result: &str) -> RPS {
        match result {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("Invalid theirs result, {}", result),
        }
    }

    fn from_outcome(theirs: RPS, outcome: Outcome) -> RPS {
        FromPrimitive::from_i32((theirs as i32 + outcome as i32 - 1 + 3) % 3).unwrap()
    }

    fn points(&self) -> i32 {
        (*self as i32) + 1
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn from_results(result: &str) -> Outcome {
        match result {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome result, {}", result),
        }
    }

    fn points(&self) -> i32 {
        (*self as i32) * 3
    }
}

struct Match {
    yours: RPS,
    outcome: Outcome,
}

impl Match {
    fn from_results(theirs: RPS, yours: RPS) -> Match {
        Match {
            yours,
            outcome: Match::outcome(theirs, yours),
        }
    }

    fn from_outcome(theirs: RPS, outcome: Outcome) -> Match {
        Match {
            yours: RPS::from_outcome(theirs, outcome),
            outcome,
        }
    }

    fn outcome(theirs: RPS, yours: RPS) -> Outcome {
        FromPrimitive::from_i32((yours - theirs + 1 + 3) % 3).unwrap()
    }

    fn score(&self) -> i32 {
        return self.yours.points() + self.outcome.points();
    }
}

fn parse_input_part1(contents: &str) -> Vec<Match> {
    contents
        .split("\n")
        .map(|list| {
            let results: Vec<&str> = list.split(" ").collect();
            let theirs = RPS::from_theirs(results[0]);
            let yours = RPS::from_yours(results[1]);
            return Match::from_results(theirs, yours);
        })
        .collect()
}

fn parse_input_part2(contents: &str) -> Vec<Match> {
    contents
        .split("\n")
        .map(|list| {
            let results: Vec<&str> = list.split(" ").collect();
            let theirs = RPS::from_theirs(results[0]);
            let outcome = Outcome::from_results(results[1]);
            return Match::from_outcome(theirs, outcome);
        })
        .collect()
}

fn solve(parsed: &Vec<Match>) -> i32 {
    parsed.iter().map(|m| m.score()).sum()
}

pub fn main() {
    let input = read_input(false);
    let parsed_part1 = parse_input_part1(&input);
    let solution_part1 = solve(&parsed_part1);
    println!("Part 1: {}", solution_part1);
    let parsed_part2 = parse_input_part2(&input);
    let solution_part2 = solve(&parsed_part2);
    println!("Part 2: {}", solution_part2)
}
