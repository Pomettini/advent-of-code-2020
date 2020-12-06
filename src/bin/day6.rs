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

    let answers = chunks.into_iter().fold(0, |mut acc, chunk| {
        let chunk_set: Vec<HashSet<char>> = chunk
            .iter()
            .map(|line| line.chars().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>();

        let unique_set: HashSet<char> = chunk_set
            .clone()
            .into_iter()
            .skip(1)
            .fold(chunk_set.clone()[0].clone(), |acc, set| {
                acc.intersection(&set).into_iter().cloned().collect()
            });

        acc += unique_set.len();
        acc
    });

    println!("The solution for the second problem is {}", answers);
}
