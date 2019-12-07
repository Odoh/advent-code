use advent::InputSnake;
use advent::cpu::IntcodeComputer;

fn part_one() {
    let input = InputSnake::new("input");
    let mut cpu = IntcodeComputer::from(&input.no_snake());

    cpu.input(1);
    cpu.run();
    loop {
        let output = cpu.output();
        if output != 0 {
            println!("Part One: {:?}", output);
            return
        }
    }
}

fn part_two() {
    let input = InputSnake::new("input");
    let mut cpu = IntcodeComputer::from(&input.no_snake());

    cpu.input(5);
    cpu.run();
    loop {
        let output = cpu.output();
        if output != 0 {
            println!("Part Two: {:?}", output);
            return
        }
    }
}

fn main() {
    part_one();
    part_two();
}

