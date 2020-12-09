fn main() {
    let input = include_str!("../../inputs/day9.txt");

    let mut preamble_list: Vec<usize> = input
        .lines()
        .take(25)
        .map(|line| line.parse().unwrap())
        .collect();

    fn is_preamble(list: &Vec<usize>, number: usize) -> bool {
        list.into_iter()
            .find(|&&a| {
                list.into_iter()
                    .find(|&&b| a != b && a + b == number)
                    .is_some()
            })
            .is_some()
    }

    let first: usize = input
        .lines()
        .skip(25)
        .map(|line| line.parse::<usize>().unwrap())
        .filter(|number| {
            let result = !is_preamble(&preamble_list, *number);
            preamble_list.remove(0);
            preamble_list.push(*number);
            result
        })
        .collect::<Vec<usize>>()
        .first()
        .unwrap()
        .to_owned();

    println!("The solution for the first problem is {}", first);

    assert!(first == 90433990);
}
