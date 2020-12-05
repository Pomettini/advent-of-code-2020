use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day1.txt");

    let numbers: Vec<i64> = input
        .lines()
        .map(|line| {
            line.parse()
                .expect("A line in the input file is not a valid number")
        })
        .collect();

    let first = numbers
        .iter()
        .tuple_combinations()
        .find(|&(a, b)| a + b == 2020)
        .expect("Cannot find a combination of two numbers that return 2020");

    let second = numbers
        .iter()
        .tuple_combinations()
        .find(|&(a, b, c)| a + b + c == 2020)
        .expect("Cannot find a combination of three numbers that return 2020");

    println!("The solution for the first part is {0}", first.0 * first.1);

    println!(
        "The solution for the second part is {0}",
        second.0 * second.1 * second.2
    );
}
