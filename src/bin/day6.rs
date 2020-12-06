use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day6.txt");

    let chunks: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|chunk| chunk.split_ascii_whitespace().collect())
        .collect();

    let mut unique_answers = 0;

    for chunk in &chunks {
        let unique: HashSet<char> = chunk.iter().flat_map(|line| line.chars()).collect();
        unique_answers += unique.len();
    }

    println!("The solution for the first problem is {}", unique_answers);

    let mut answers = 0;

    for chunk in &chunks {
        let mut t: Vec<HashSet<char>> = Vec::new();

        for line in chunk {
            let answer: HashSet<char> = line.chars().collect();
            t.push(answer);
        }

        let mut temp = t.first().unwrap().clone();

        for chunk in t.iter().skip(1) {
            temp = temp.intersection(&chunk).into_iter().cloned().collect();
        }

        answers += temp.len();
    }

    println!("The solution for the second problem is {}", answers);
}
