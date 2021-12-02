fn main() {
    let input = include_str!("../../inputs/day9.txt");

    let lines: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut preamble_list: Vec<usize> = lines.iter().take(25).map(|l| l.to_owned()).collect();

    fn is_not_preamble(list: &[usize], number: usize) -> bool {
        list.iter()
            .find(|&&a| list.iter().any(|&b| a != b && a + b == number))
            .is_none()
    }

    let first: (usize, usize) = input
        .lines()
        .skip(25)
        .map(|line| line.parse::<usize>().unwrap())
        .enumerate()
        .filter(|(_i, number)| {
            let result = is_not_preamble(&preamble_list, *number);
            preamble_list.remove(0);
            preamble_list.push(*number);
            result
        })
        .collect::<Vec<(usize, usize)>>()
        .first()
        .unwrap()
        .to_owned();

    println!("The solution for the first problem is {}", first.1);

    assert!(first.1 == 90433990);

    fn find_in_range(list: Vec<usize>, start: usize, end: usize, item: usize) -> Option<usize> {
        let range: Vec<usize> = list.into_iter().skip(start).take(end).collect();

        if range.iter().sum::<usize>() == item {
            Some(*range.iter().min()? + *range.iter().max()?)
        } else {
            None
        }
    }

    let second: usize = (0..first.0)
        .into_iter()
        .find_map(|skip| {
            (0..25)
                .into_iter()
                .find_map(|take| find_in_range(lines.clone(), skip, take, first.1))
        })
        .unwrap();

    println!("The solution to the second problem is {}", second);

    assert!(second == 11691646);
}
