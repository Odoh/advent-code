use advent::InputSnake;
use advent::cpu;
use env_logger;
use futures::executor;

fn part_one() {
    let input = InputSnake::new("input");
    let mut cpu = cpu::IntcodeComputer::from(&input.no_snake());

    cpu.send_input(1);
    cpu = executor::block_on(cpu.run());
    let output = cpu.recv_output();

    println!("Part One: {:?}", output);
}

fn part_two() {
    let input = InputSnake::new("input");
    let mut cpu = cpu::IntcodeComputer::from(&input.no_snake());

    cpu.send_input(2);
    cpu = executor::block_on(cpu.run());
    let output = cpu.recv_output();

    println!("Part Two: {:?}", output);
}

fn main() {
    env_logger::init();

    part_one();
    part_two();
}
