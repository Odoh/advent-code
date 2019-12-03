use advent::InputSnake;

fn fuel_required(mut mass: i32) -> i32 {
    let mut total = 0;
    loop {
        mass = mass / 3 - 2;
        if mass <= 0 {
            return total;
        }
        total += mass
    }
}

fn part_one() {
    let input = InputSnake::new("input");
    let fuel: i32 = input.snake()
        .map(|l| l.parse::<i32>().unwrap())
        .map(|mass| mass / 3 - 2)
        .sum();
    println!("Part One: {}", fuel);
}

fn part_two() {
    let input = InputSnake::new("input");
    let fuel: i32 = input.snake()
        .map(|l| l.parse::<i32>().unwrap())
        .map(|mass| fuel_required(mass))
        .sum();
    println!("Part Two: {}", fuel)
}

fn main() {
    part_one();
    part_two();
}
