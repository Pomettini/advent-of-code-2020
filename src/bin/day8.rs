type Instructions<'a> = Vec<(&'a str, i32)>;
type State = (usize, i32);

fn main() {
    let input = include_str!("../../inputs/day8.txt");

    let program: Instructions =
        input
            .lines()
            .map(|line| line.split(' '))
            .fold(Vec::new(), |mut line, mut values| {
                line.push((
                    values.next().unwrap(),
                    values.next().unwrap().parse().unwrap(),
                ));
                line
            });

    fn run_program(program: &Instructions) -> State {
        let mut pc: usize = 0;
        let mut acc: i32 = 0;
        let mut visited: Vec<usize> = Vec::new();

        loop {
            if pc == program.len() {
                return (pc, acc);
            }

            if visited.contains(&pc) {
                break;
            }

            visited.push(pc);

            match program[pc].0 {
                "acc" => {
                    acc += program[pc].1;
                }
                "jmp" => {
                    pc = ((pc as i32) + (program[pc].1 as i32)) as usize;
                    continue;
                }
                _ => (),
            }

            pc += 1;
        }

        (pc, acc)
    }

    let first = run_program(&program).1;

    println!("The solution for the first problem is {}", first);

    assert!(first == 1723);

    let combinations: Vec<State> = (0..program.len())
        .filter(|&i| program[i].0 != "acc")
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
        .filter(|combination| combination.0 == program.len())
        .collect::<Vec<State>>()
        .first()
        .unwrap()
        .1;

    println!("The solution for the second problem is {}", second);

    assert!(second == 846);
}
