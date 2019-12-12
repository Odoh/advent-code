use advent::InputSnake;

type Asteroid = (f64, f64);

fn parse_asteroids(input: InputSnake) -> Vec<Asteroid> {
    input.snake()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(|(x, _)| (x as f64, y as f64))
            .collect::<Vec<Asteroid>>())
        .collect()
}

fn angle_between_asteroids((x1, y1): Asteroid, (x2, y2): Asteroid) -> f64 {
    let degrees = (x2 - x1).atan2(y1 - y2).to_degrees();
    if degrees < 0. {
        degrees + 360.
    } else {
        degrees
    }
}

fn detectable_asteroids(asteroid: Asteroid, all_asteroids: &[Asteroid]) -> Vec<(f64, Asteroid)> {
    let mut angle_and_asteroid = all_asteroids.iter()
        .map(|&other_asteroid| (angle_between_asteroids(asteroid, other_asteroid), other_asteroid))
        .collect::<Vec<(f64, Asteroid)>>();
    angle_and_asteroid.sort_by(|(a,_),(b,_)| a.partial_cmp(b).expect("No NaNs"));
    angle_and_asteroid.dedup_by(|(a,_),(b,_)| a == b);
    angle_and_asteroid
}

fn part_one() {
    let input = InputSnake::new("input");
    let asteroids = parse_asteroids(input);

    let most_asteroids = asteroids.iter()
        .map(|&asteroid| (asteroid, detectable_asteroids(asteroid, &asteroids[..]).len()))
        .max_by_key(|&(_, detects)| detects)
        .unwrap();

    println!("Part One: {:?}", most_asteroids);
}

fn part_two() {
    let input = InputSnake::new("input");
    let asteroids = parse_asteroids(input);

    let monitoring = (22.0, 25.0);  // part 1
    let detectables = detectable_asteroids(monitoring, &asteroids[..]);
    let two_hundredth = detectables.iter().nth(199).unwrap();

    println!("Part Two: {:?}", (two_hundredth.1).0 * 100. + (two_hundredth.1).1);
}

fn main() {
    part_one();
    part_two();
}
