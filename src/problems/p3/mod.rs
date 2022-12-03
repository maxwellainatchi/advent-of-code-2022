use std::{collections::BTreeSet, fs, path::Path};

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

fn priority(char: char) -> usize {
    let ascii = char as usize;
    match ascii {
        (97..=122) => ascii - 97 + 1, // lowercase
        (65..=90) => ascii - 65 + 27, // uppercase
        _ => panic!("Only letters allowed"),
    }
}

#[derive(Debug)]
struct Rucksack<'a> {
    compartment1: &'a str,
    compartment2: &'a str,
    original: &'a str,
}

impl<'a> Rucksack<'a> {
    fn parse(line: &'a str) -> Rucksack {
        let compartments = line.split_at(line.len() / 2);
        Rucksack {
            compartment1: compartments.0,
            compartment2: compartments.1,
            original: line,
        }
    }
}

fn parse(text: &str) -> Vec<Rucksack> {
    text.split("\n").map(|line| Rucksack::parse(line)).collect()
}

fn to_set(compartment: &str) -> BTreeSet<char> {
    compartment.chars().fold(BTreeSet::new(), |mut set, c| {
        set.insert(c);
        return set;
    })
}

fn solve_part1(rucksacks: &Vec<Rucksack>) -> usize {
    let mut all_solutions: Vec<usize> = vec![];
    for rucksack in rucksacks {
        let set1 = to_set(rucksack.compartment1);
        let set2 = to_set(rucksack.compartment2);
        let isec = set1.intersection(&set2);
        all_solutions.push(priority(*isec.min().unwrap()))
    }
    all_solutions.iter().sum()
}

fn solve_part2(rucksacks: &Vec<Rucksack>) -> usize {
    let mut all_badges: Vec<usize> = vec![];
    for i in (0..rucksacks.len()).step_by(3) {
        let set1 = to_set(rucksacks[i].original);
        let set2 = to_set(rucksacks[i + 1].original);
        let set3 = to_set(rucksacks[i + 2].original);
        let isec = set1.intersection(&set2).filter(|char| set3.contains(char));
        all_badges.push(priority(*isec.min().unwrap()))
    }
    all_badges.iter().sum()
}

pub fn main() {
    let input = read_input(false);
    let parsed = parse(&input);
    let solution_part1 = solve_part1(&parsed);
    println!("Part 1: {}", solution_part1);
    let solution_part2 = solve_part2(&parsed);
    println!("Part 2: {}", solution_part2)
}
