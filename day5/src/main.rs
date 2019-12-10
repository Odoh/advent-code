use futures;
use advent::InputSnake;
use advent::cpu::IntcodeComputer;

fn part_one() {
    let input = InputSnake::new("input");
    let mut cpu = IntcodeComputer::from(&input.no_snake());

    cpu.send_input(1);
    cpu = futures::executor::block_on(cpu.run());
    loop {
        let output = cpu.recv_output();
        if output != 0 {
            println!("Part One: {:?}", output);
            return
        }
    }
}

fn part_two() {
    let input = InputSnake::new("input");
    let mut cpu = IntcodeComputer::from(&input.no_snake());

    cpu.send_input(5);
    cpu = futures::executor::block_on(cpu.run());
    loop {
        let output = cpu.recv_output();
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

