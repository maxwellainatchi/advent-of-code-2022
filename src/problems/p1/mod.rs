use std::{fs, path::Path};

fn read_input() -> String {
    let file_path = Path::new(file!());
    let path = file_path.with_file_name("input.txt");
    let contents = fs::read_to_string(path).unwrap();
    return contents;
}

fn parse_input(contents: String) -> Vec<Vec<i32>> {
    contents
        .split("\n\n")
        .map(|list| {
            list.split("\n")
                .map(|number| match number.parse() {
                    Ok(val) => val,
                    Err(error) => panic!("{} {}", number, error),
                })
                .collect()
        })
        .collect()
}

fn get_elf_sums(groups: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sums: Vec<i32> = groups.iter().map(|group| group.iter().sum()).collect();
    sums.sort_by(|a, b| b.cmp(a));
    return sums;
}

/** Calculate the greatest sum of the "calories" of each elf's collection of food */
fn solve_part1(sums: &Vec<i32>) -> i32 {
    *sums.first().unwrap()
}

/** Calculate the sum of the top 3 greatest sums instead of the top 1 */
fn solve_part2(sums: &Vec<i32>) -> i32 {
    sums.iter().take(3).sum()
}

pub fn main() {
    let contents = read_input();
    let groups = parse_input(contents);
    let sums = get_elf_sums(groups);
    let solution_part1 = solve_part1(&sums);
    println!("Part 1: {}", solution_part1);
    let solution_part2 = solve_part2(&sums);
    println!("Part 2: {}", solution_part2)
}
