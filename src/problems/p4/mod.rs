use std::{fs, path::Path};

struct Area {
    start: i32,
    end: i32,
}
struct Pair(Area, Area);

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

fn parse_range(range: &str) -> Area {
    let mut bounds = range.split("-").map(|b| b.parse::<i32>().unwrap());
    return Area {
        start: bounds.next().unwrap(),
        end: bounds.next().unwrap(),
    };
}

fn parse(input: &str) -> Vec<Pair> {
    input
        .split("\n")
        .map(|pair| {
            let mut ranges = pair.split(",").map(|r| parse_range(r));
            return Pair(ranges.next().unwrap(), ranges.next().unwrap());
        })
        .collect()
}

fn one_contains_other(pair: &Pair) -> bool {
    let diffs = (pair.0.start - pair.1.start, pair.0.end - pair.1.end);
    let signs = (diffs.0.signum(), diffs.1.signum());
    return signs.0 * signs.1 <= 0;
}

fn overlaps(pair: &Pair) -> bool {
    let diffs = ((pair.1.start - pair.0.end), (pair.1.end - pair.0.start));
    let signs = (diffs.0.signum(), diffs.1.signum());
    let overlaps = signs.0 != signs.1 || signs.0 == 0;
    return overlaps;
}

fn solve(parsed: &Vec<Pair>, comparison: fn(&Pair) -> bool) -> i32 {
    return parsed.iter().filter(|pair| comparison(pair)).count() as i32;
}

pub fn main() {
    let input = read_input(false);
    let parsed = parse(&input);
    let solution_part1 = solve(&parsed, one_contains_other);
    println!("Part 1: {}", solution_part1);
    let solution_part2 = solve(&parsed, overlaps);
    println!("Part 2: {}", solution_part2);
}
