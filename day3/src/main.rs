fn is_tree(
    map_width: usize,
    lines: &[&str],
    current_line: usize,
    slope: (usize, usize),
) -> Option<()> {
    if lines
        .get(current_line * slope.1)?
        .chars()
        .nth((current_line * slope.0) % map_width)?
        == '#'
    {
        Some(())
    } else {
        None
    }
}

fn main() {
    let input = include_str!("input.txt");

    let lines: Vec<&str> = input.lines().collect();

    let map_width = lines
        .get(0)
        .expect("First line of the input file is empty")
        .len();

    let mut count = 0;

    for line in 0..lines.len() {
        if is_tree(map_width, &lines, line, (3, 1)).is_some() {
            count += 1;
        }
    }

    println!("The solution for the first problem is {}", count);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut total_count: [usize; 5] = [0; 5];

    for slope_type in 0..slopes.len() {
        for line in 0..lines.len() {
            if is_tree(map_width, &lines, line, slopes[slope_type]).is_some() {
                total_count[slope_type] += 1;
            }
        }
    }

    count = total_count.iter().product();

    println!("The solution for the second problem is {}", count);
}
