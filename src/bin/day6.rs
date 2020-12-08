// #![feature(iterator_fold_self)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day6.txt");

    let people_group: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|chunk| chunk.split_ascii_whitespace().collect())
        .collect();

    let unique_answers = people_group.clone().into_iter().fold(0, |a, b| {
        let unique: HashSet<char> = b.iter().flat_map(|line| line.chars()).collect();
        a + unique.len()
    });

    println!("The solution for the first problem is {}", unique_answers);

    assert!(unique_answers == 6596);

    /*
    let answers = people_group
        .into_iter()
        .fold(0, |unique_answers, person_answers| {
            unique_answers
                + person_answers
                    .iter()
                    .map(|line| line.chars().collect::<HashSet<char>>())
                    .collect::<Vec<HashSet<char>>>()
                    .into_iter()
                    .fold_first(|unique_set_elements, unique_set| {
                        unique_set_elements
                            .intersection(&unique_set)
                            .into_iter()
                            .cloned()
                            .collect()
                    })
                    .unwrap_or_default()
                    .len()
        });

    println!("The solution for the second problem is {}", answers);

    assert!(answers == 3219);
    */
}
