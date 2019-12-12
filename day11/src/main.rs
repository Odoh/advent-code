use advent::InputSnake;
use advent::cpu::{IntcodeComputer, Proxy};
use std::collections::{HashMap, HashSet};

const STARTING_LOCATION: (i64, i64) = (0, 0);
const STARTING_DIRECTION: Direction = Direction::Up;
const DEFAULT_COLOR: Color = Color::Black;

#[derive(Copy, Clone, Debug)]
enum Color {
    Black,
    White,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn turn (self, direction: Direction) -> Direction {
        match (self, direction) {
            (Direction::Up, Direction::Left) => Direction::Left,
            (Direction::Up, Direction::Right) => Direction::Right,
            (Direction::Down, Direction::Left) => Direction::Right,
            (Direction::Down, Direction::Right) => Direction::Left,
            (Direction::Left, Direction::Left) => Direction::Down,
            (Direction::Left, Direction::Right) => Direction::Up,
            (Direction::Right, Direction::Left) => Direction::Up,
            (Direction::Right, Direction::Right) => Direction::Down,
            (_, _) => panic!("Unsupported turn {:?} {:?}", self, direction),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Coord {
    inner: (i64, i64),
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Coord {
        Coord {
            inner: (x, y)
        }
    }

    pub fn walk(self, direction: Direction) -> Coord {
        match direction {
            Direction::Up => Coord::new(self.inner.0, self.inner.1 + 1),
            Direction::Down => Coord::new(self.inner.0, self.inner.1 - 1),
            Direction::Left => Coord::new(self.inner.0 - 1, self.inner.1),
            Direction::Right => Coord::new(self.inner.0 + 1, self.inner.1),
        }
    }
}

struct Hull {
    coord_to_color: HashMap<Coord, Color>,
}

impl Hull {
    pub fn new(initial_panel_color: Color) -> Hull {
        let mut coord_to_color = HashMap::new();
        coord_to_color.insert(Coord::new(STARTING_LOCATION.0, STARTING_LOCATION.1), initial_panel_color);
        Hull {
            coord_to_color,
        }
    }

    pub fn color(&mut self, coord: Coord) -> Color {
        *self.coord_to_color.entry(coord).or_insert(DEFAULT_COLOR)
    }

    pub fn paint(&mut self, coord: Coord, color: Color) {
        self.coord_to_color.insert(coord, color);
    }

    pub fn print(&self, max_size: i64) {
        for x in -max_size..max_size {
            for y in -max_size..max_size {
                let coord: Coord = Coord::new(x as i64, y as i64);
                let color = self.coord_to_color.get(&coord).or(Some(&Color::Black)).unwrap();
                match color {
                    Color::Black => print!("▓"),
                    Color::White => print!("░"),
                }
            }
            println!("");
        }
    }
}

struct Robot {
    cpu: Option<IntcodeComputer>,
    location: Coord,
    direction: Direction,
    locations_painted: HashSet<Coord>,
}

impl Robot {
    pub fn new(memory: &str) -> Robot {
        Robot {
            cpu: Some(IntcodeComputer::from(memory)),
            location: Coord::new(STARTING_LOCATION.0, STARTING_LOCATION.1),
            direction: STARTING_DIRECTION,
            locations_painted: HashSet::new(),
        }
    }

    pub fn panels_painted(&self) -> i64 {
        self.locations_painted.len() as i64
    }

    pub fn run(&mut self, hull: &mut Hull) {
        let mut runtime = tokio::runtime::Runtime::new().expect("Initialized runtime");

        runtime.block_on(async {
            let mut cpu = self.cpu.take().expect("CPU to exist");
            let mut cpu_proxy = cpu.proxy();
            futures::future::join(cpu.run(), self.run_steps(&mut cpu_proxy, hull)).await;
        });
    }

    async fn run_steps(&mut self, cpu_proxy: &mut Proxy, hull: &mut Hull) {
        loop {
            let direction = self.direction;
            let location = self.location;
            let square_color = hull.color(location);
//            println!("Step {:?} {:?} {:?}", direction, location, square_color);

            cpu_proxy.send(square_color.code()).await;
            if let Some(paint_color) = cpu_proxy.recv().await.map(|code| Color::from(code)) {
                let turn_direction = Direction::from(cpu_proxy.recv().await.unwrap());

                hull.paint(location, paint_color);
                self.locations_painted.insert(location);
                self.direction = direction.turn(turn_direction);
                self.location = location.walk(self.direction);
            } else {
                return
            }
        }
    }
}

impl Color {
    pub fn from(code: i64) -> Color {
        match code {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Unhandled color code: {}", code),
        }
    }

    pub fn code(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl Direction {
    pub fn from(code: i64) -> Direction {
        match code {
            0 => Direction::Left,
            1 => Direction::Right,
            _ => panic!("Unhandled direction code"),
        }
    }
}

fn part_one() {
    let input = InputSnake::new("input");
    let mut robot = Robot::new(&input.no_snake());
    let mut hull = Hull::new(Color::Black);

    robot.run(&mut hull);

    println!("Part One: {:?}", robot.panels_painted());
}

fn part_two() {
    let input = InputSnake::new("input");
    let mut robot = Robot::new(&input.no_snake());
    let mut hull = Hull::new(Color::White);

    robot.run(&mut hull);
    let max_size = robot.panels_painted();

    println!("Part Two:");
    hull.print(50);
}

fn main() {
    env_logger::init();
    part_one();
    part_two();
}
