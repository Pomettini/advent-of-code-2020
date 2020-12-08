fn main() {
    let input = include_str!("../../inputs/day8.txt");

    let program: Vec<(&str, i32)> = input.lines().map(|line| line.split(" ")).fold(
        Vec::<(&str, i32)>::new(),
        |mut line, mut values| {
            line.push((
                values.next().unwrap(),
                values.next().unwrap().parse().unwrap(),
            ));
            line
        },
    );

    let mut pc: i32 = 0;
    let mut acc: i32 = 0;
    let mut visited: Vec<i32> = Vec::new();

    loop {
        if visited.contains(&pc) {
            break;
        }

        visited.push(pc);

        match &program[pc as usize].0 {
            &"acc" => {
                acc += program[pc as usize].1;
            }
            &"jmp" => {
                pc += program[pc as usize].1;
                continue;
            }
            _ => (),
        }

        pc += 1;
    }

    println!("The solution for the first problem is {}", acc);

    assert!(acc == 1723);
}
