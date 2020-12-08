fn main() {
    let input = include_str!("../../inputs/day5.txt");

    let lines: Vec<&str> = input.lines().collect();

    let mut results: Vec<usize> = Vec::new();

    for line in lines {
        let mut range: Vec<usize> = (0..128).collect();

        let mut iter = line.chars();

        for _ in 0..7 {
            match iter.next() {
                Some('F') => {
                    range = range.drain(0..(range.len() / 2)).collect();
                }
                Some('B') => {
                    range = range.drain((range.len() / 2)..range.len()).collect();
                }
                _ => {}
            }
        }

        let row = range.first().unwrap();

        let mut range: Vec<usize> = (0..8).collect();

        for _ in 0..3 {
            match iter.next() {
                Some('L') => {
                    range = range.drain(0..(range.len() / 2)).collect();
                }
                Some('R') => {
                    range = range.drain((range.len() / 2)..range.len()).collect();
                }
                _ => {}
            }
        }

        let column = &range.first().unwrap();

        let result = *row * 8 + *column;

        results.push(result);
    }

    let first = results.iter().max().unwrap().to_owned();

    println!("The solution for the first problem is {}", first);

    assert!(first == 987);

    results.sort_unstable();

    let second = results
        .iter()
        .enumerate()
        .find(|(a, b)| *b + 1 != results[a + 1])
        .unwrap()
        .1
        + 1;

    println!("The solution for the second problem is {}", second);

    assert!(second == 603);
}
