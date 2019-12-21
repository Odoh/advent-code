use advent::InputSnake;
use advent::cpu::IntcodeComputer;
use advent::grid::{Grid, Coord, DrawType};
use std::time::Duration;

#[derive(Debug)]
enum Movement {
    North,
    South,
    West,
    East,
}

enum DroidStatus {
    /// The repair droid hit a wall. Its position has not changed.
    Wall,
    /// The repair droid has moved one step in the requested direction.
    Moved,
    /// The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
    Oxygen,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Start,
    Wall,
    Droid,
    Path,
    Oxygen,
}

impl Tile {
    pub fn draw(coord: Coord, tile: Option<&Tile>) -> Option<char> {
        if let Some(tile) = tile {
            match tile {
                Tile::Start => Some('S'),
                Tile::Wall => Some('#'),
                Tile::Droid => Some('D'),
                Tile::Path => Some('.'),
                Tile::Oxygen => Some('O'),
            }
        } else {
            None
        }
    }
}

struct Droid {
    cpu: IntcodeComputer,
}

impl Droid {
    pub fn new(memory: &str) -> Droid {
        Droid {
            cpu: IntcodeComputer::from(memory),
        }
    }

    pub fn run(&mut self) {
        let mut runtime = tokio::runtime::Runtime::new().expect("Runtime to init");
        let mut proxy = self.cpu.proxy();
        let mut grid = Grid::new();
        grid.init_curses();

        let mut coord = (25, 25);
        grid.add_entry(coord, Tile::Start);
        grid.draw(Tile::draw);

        // depth first search
        let mut movement_path: Vec<Movement> = Vec::new();

        let droid_io = async {
            loop {
                // move in a direction we haven't been
                let mut movement = None;
                for m in (1..=4).map(|code| Movement::from(code)) {
                    if grid.entry(m.next_coord(coord)).is_none() {
                        movement = Some(m);
                    }
                };
                // if all movement locations have been visited, backtrack
                if movement.is_none() {
                    movement = movement_path.pop().map(|m| m.opposite());
                }

                let movement = movement.unwrap();
                let next_coord = movement.next_coord(coord);

                proxy.send(movement.code()).await;
                match proxy.recv().await {
                    None => return,
                    Some(status) => {
                        match DroidStatus::from(status) {
                            DroidStatus::Wall => {
                                grid.add_entry(next_coord, Tile::Wall);
                            },
                            DroidStatus::Moved => {
                                if grid.entry(next_coord).is_none() {
                                    movement_path.push(movement);
                                }

                                if *grid.entry(coord).unwrap() != Tile::Start {
                                    grid.add_entry(coord, Tile::Path);
                                }
                                if grid.entry(next_coord).is_none() ||
                                   *grid.entry(next_coord).unwrap() != Tile::Start {
                                    grid.add_entry(next_coord, Tile::Droid);
                                }

                                coord = next_coord;
                            },
                            DroidStatus::Oxygen => {
                                if grid.entry(next_coord).is_none() {
                                    movement_path.push(movement);
                                }
                                grid.add_entry(coord, Tile::Path);
                                grid.add_entry(next_coord, Tile::Oxygen);

                                coord = next_coord;
                                grid.draw(Tile::draw);
                                let path_len = format!("moment_path: {}", movement_path.len());
                                match grid.draw_type() {
                                    DrawType::Curses(w) => {
                                        w.mvprintw(0, 0, path_len);
                                        w.refresh();
                                    },
                                    DrawType::StdOut => println!("{}", path_len),
                                };
                                return;
                            }
                        };
                        grid.draw(Tile::draw);
                    },
                }
            }
        };
        runtime.block_on(futures::future::join(self.cpu.run(), droid_io));
    }
}

impl Movement {
    pub fn from(code: i64) -> Movement {
        match code {
            1 => Movement::North,
            2 => Movement::South,
            3 => Movement::West,
            4 => Movement::East,
            _ => panic!("Unhandled movement code {}", code),
        }
    }

    pub fn opposite(&self) -> Movement {
        match self {
            Movement::North => Movement::South,
            Movement::South => Movement::North,
            Movement::West => Movement::East,
            Movement::East => Movement::West,
        }
    }

    pub fn code(&self) -> i64 {
        match self {
            Movement::North => 1,
            Movement::South => 2,
            Movement::West => 3,
            Movement::East => 4,
        }
    }

    pub fn next_coord(&self, coord: Coord) -> Coord {
        match self {
            Movement::North => (coord.0, coord.1 - 1),
            Movement::South => (coord.0, coord.1 + 1),
            Movement::West => (coord.0 - 1, coord.1),
            Movement::East => (coord.0 + 1, coord.1),
        }
    }
}

impl DroidStatus {
    pub fn from(code: i64) -> DroidStatus {
        match code {
            0 => DroidStatus::Wall,
            1 => DroidStatus::Moved,
            2 => DroidStatus::Oxygen,
            _ => panic!("Unhandled status code {}", code),
        }
    }
}

fn part_one() {
    let input = InputSnake::new("input");
    let mut droid = Droid::new(&input.no_snake());

    droid.run();

    println!("Part One: {:?}", 1);
}

fn part_two() {
    let input = InputSnake::new("input");
    println!("Part Two: {:?}", 2);
}

fn main() {
    env_logger::init();

    part_one();
    part_two();
}
