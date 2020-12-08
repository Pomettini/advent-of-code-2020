fn main() {
    type Instructions<'a> = Vec<(&'a str, i32)>;

    let input = include_str!("../../inputs/day8.txt");

    let program: Instructions = input.lines().map(|line| line.split(" ")).fold(
        Vec::<(&str, i32)>::new(),
        |mut line, mut values| {
            line.push((
                values.next().unwrap(),
                values.next().unwrap().parse().unwrap(),
            ));
            line
        },
    );

    fn run_program(program: &Instructions) -> Option<(i32, i32)> {
        let mut pc: i32 = 0;
        let mut acc: i32 = 0;
        let mut visited: Vec<i32> = Vec::new();

        loop {
            if (pc == program.len() as i32) {
                return Some((pc, acc));
            }

            if pc < 0 || pc > (program.len() - 1) as i32 {
                return None;
            }

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

        Some((pc, acc))
    }

    dbg!(run_program(&program));

    let first = run_program(&program).unwrap().1;

    println!("The solution for the first problem is {}", first);

    assert!(first == 1723);

    let combinations: Vec<Option<(i32, i32)>> = (0..program.len())
        .filter(|&i| &program[i].0 != &"acc")
        .map(|i| {
            let mut clone = program.clone();
            match clone[i].0 {
                "nop" => clone[i].0 = "jmp",
                "jmp" => clone[i].0 = "nop",
                _ => (),
            }
            run_program(&clone)
        })
        .collect();

    let second: i32 = combinations
        .into_iter()
        .filter(|x| x.is_some() && x.unwrap().0 == program.len() as i32)
        .collect::<Vec<Option<(i32, i32)>>>()
        .first()
        .unwrap()
        .unwrap()
        .1;

    println!("The solution for the second problem is {}", second);

    assert!(second == 846);
}
