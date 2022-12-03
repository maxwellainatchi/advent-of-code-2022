use std::{fs, ops::Sub, path::Path};

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

#[derive(Clone, Copy, Debug)]
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

    fn points(&self) -> i32 {
        (*self as i32) + 1
    }
}

struct Match {
    yours: RPS,
    theirs: RPS,
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn points(&self) -> i32 {
        (*self as i32) * 3
    }
}

impl Match {
    fn outcome(&self) -> Outcome {
        match self.yours - self.theirs {
            0 => Outcome::Draw,
            1 | -2 => Outcome::Win,
            2 | -1 => Outcome::Loss,
            _ => panic!("Invalid input"),
        }
    }

    fn score(&self) -> i32 {
        return self.yours.points() + self.outcome().points();
    }
}

fn parse_input(contents: String) -> Vec<Match> {
    contents
        .split("\n")
        .map(|list| {
            let results: Vec<&str> = list.split(" ").collect();
            let theirs = RPS::from_theirs(results[0]);
            let yours = RPS::from_yours(results[1]);
            return Match { theirs, yours };
        })
        .collect()
}

fn solve_part1(parsed: &Vec<Match>) -> i32 {
    parsed.iter().map(|m| m.score()).sum()
}

fn solve_part2(parsed: &Vec<Match>) -> i32 {
    0
}

pub fn main() {
    let input = read_input(false);
    let parsed = parse_input(input);
    let solution_part1 = solve_part1(&parsed);
    println!("Part 1: {}", solution_part1);
    let solution_part2 = solve_part2(&parsed);
    println!("Part 2: {}", solution_part2)
}
