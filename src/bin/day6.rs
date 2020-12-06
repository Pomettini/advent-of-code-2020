use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day6.txt");

    let lines: Vec<String> = input
        .split("\n\n")
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(|l| l.chars())
                .collect()
        })
        .collect();

    let mut unique_answers = 0;

    for line in lines {
        let unique: HashSet<char> = line.chars().collect();
        unique_answers += unique.len();
    }

    println!("The solution for the second problem is {}", unique_answers);
}
