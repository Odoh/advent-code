use advent::InputSnake;

enum OpCode {
    Addition,
    Multiplication,
    Halt,
}

impl OpCode {
    pub fn new(code: i32) -> OpCode {
        match code {
            1 => OpCode::Addition,
            2 => OpCode::Multiplication,
            99 => OpCode::Halt,
            _ => panic!("Unhandled OpCode {}", code),
        }
    }
}

fn run_program(program: &mut Vec<i32>) {
    for i in (0..program.len()).step_by(4) {
        let op_code = OpCode::new(program[i]);
        let read_1 = program[i + 1] as usize;
        let read_2 = program[i + 2] as usize;
        let write = program[i + 3] as usize;

        match op_code {
            OpCode::Addition => program[write] = program[read_1] + program[read_2],
            OpCode::Multiplication => program[write] = program[read_1] * program[read_2],
            OpCode::Halt => return,
        }
    }
}

fn run_program_for_value(original_program: &Vec<i32>, value: i32, range: std::ops::RangeInclusive<i32>) -> (i32, i32) {
    for noun in range.clone() {
        for verb in range.clone() {
            let mut program = original_program.clone();
            program[1] = noun;
            program[2] = verb;
            run_program(&mut program);

            if program[0] == value {
                return (noun, verb);
            }
        }
    }
    panic!("Unable to find (noun, verb) to result in value {}", value);
}

fn part_one() {
    let mut input = InputSnake::new("input").no_snake()
        .split(",")
        .map(|s| s.parse::<i32>().expect(s))
        .collect::<Vec<i32>>();

    // restore "1202 program alarm" state
    input[1] = 12;
    input[2] = 2;
    run_program(&mut input);

    println!("Part One: {:?}", input[0]);
}

fn part_two() {
    let input = InputSnake::new("input").no_snake()
        .split(",")
        .map(|s| s.parse::<i32>().expect(s))
        .collect::<Vec<i32>>();

    let (noun, verb) = run_program_for_value(&input, 19690720, 0..=99);

    println!("Part Two: {:?}", 100 * noun + verb);
}

fn main() {
    part_one();
    part_two();
}
