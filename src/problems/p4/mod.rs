use std::{fs, ops::RangeInclusive, path::Path};

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

struct Elf {
    range: RangeInclusive<i32>,
}

fn parse_range(range: &str) -> RangeInclusive<i32> {
    let mut bounds = range.split("-");
    let start = bounds.next().unwrap().parse::<i32>().unwrap();
    let end = bounds.next().unwrap().parse::<i32>().unwrap();
    return RangeInclusive::new(start, end);
}

fn parse(input: &str) -> Vec<(Elf, Elf)> {
    input
        .split("\n")
        .map(|pair| {
            let mut ranges = pair.split(",");
            let r1 = parse_range(ranges.next().unwrap());
            let r2 = parse_range(ranges.next().unwrap());
            return (Elf { range: r1 }, Elf { range: r2 });
        })
        .collect()
}

fn one_contains_other(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    let start_diff = r1.start() - r2.start();
    let end_diff = r1.end() - r2.end();
    let signs = (start_diff.signum(), end_diff.signum());
    let contains = signs.0 * signs.1 <= 0;
    // println!(
    //     "{:#?} {} {:#?}: {:?}",
    //     r1,
    //     if contains {
    //         "contains"
    //     } else {
    //         "does not contain"
    //     },
    //     r2,
    //     signs
    // );
    return contains;
}

fn solve_part1(parsed: &Vec<(Elf, Elf)>) -> i32 {
    return parsed
        .iter()
        .filter(|(e1, e2)| one_contains_other(&e1.range, &e2.range))
        .count() as i32;
}

pub fn main() {
    let input = read_input(false);
    let parsed = parse(&input);
    let solution_part1 = solve_part1(&parsed);
    println!("Part 1: {}", solution_part1);
}
