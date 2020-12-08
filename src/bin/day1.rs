use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day1.txt");

    let numbers: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    let first: u32 = numbers
        .iter()
        .combinations(2)
        .find(|v| v[0] + v[1] == 2020)
        .unwrap()
        .into_iter()
        .product();

    println!("The solution for the first part is {0}", first);

    assert!(first == 1007331);

    let second: u32 = numbers
        .iter()
        .combinations(3)
        .find(|v| v[0] + v[1] + v[2] == 2020)
        .unwrap()
        .into_iter()
        .product();

    println!("The solution for the second part is {0}", second);

    assert!(second == 48914340);
}
