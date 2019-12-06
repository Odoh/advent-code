use advent::InputSnake;
use advent::cpu::IntcodeComputer;

fn part_one() {
    let input = InputSnake::new("input");
    let mut cpu = IntcodeComputer::from(&input.no_snake());
    cpu.run();
    println!("Part One: {:?}", 1);
}

fn part_two() {
    let input = InputSnake::new("input");
    println!("Part Two: {:?}", 2)
}

fn main() {
    part_one();
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
    }

    #[test]
    fn test_part_2() {
    }
}
