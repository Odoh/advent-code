use std::collections::HashMap;
use advent::InputSnake;
use advent::cpu::IntcodeComputer;
use advent::grid::{Grid, DrawType, Input, ReadProxy};
use std::time::{Instant, Duration};
use std::fs::OpenOptions;
use std::io::Write;
use std::writeln;

const PRINT_SCORE_COORD: (i32, i32) = (30, 0);
const INPUT_WIN_COORD: (i32, i32) = (31, 0);
const SCORE_COORD: (i64, i64) = (-1, 0);
const JOYSTICK_INPUT_FILE: &str = "joystick_input";

type Coord = (i64, i64);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    /// No game object appears in this tile.
    Empty,
    /// Walls are indestructible barriers.
    Wall,
    /// Blocks can be broken by the ball.
    Block,
    /// The horizontal paddle is indestructible.
    Paddle,
    /// The ball moves diagonally and bounces off objects.
    Ball,
}

struct Arcade {
    cpu: IntcodeComputer,
}

struct Screen {
    score: i64,
    grid: Grid<Tile>,
}


impl Tile {
    pub fn from(tile_id: i64) -> Tile {
        match tile_id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Unhandled tile_id {}", tile_id),
        }
    }

    pub fn char(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => 'X',
            Tile::Block => 'B',
            Tile::Paddle => '-',
            Tile::Ball => 'o',
        }
    }
}

impl Arcade {
    pub fn new(memory: &str) -> Arcade {
        Arcade {
            cpu: IntcodeComputer::from(memory),
        }
    }

    pub fn free_play(&mut self) {
        self.cpu.set_memory(0, 2);
    }

    pub fn run(&mut self) -> Screen {
        let mut runtime = tokio::runtime::Runtime::new().expect("Runtime to init");
        let mut input_proxy = self.cpu.input_proxy();
        let mut output_proxy = self.cpu.output_proxy();

        let mut screen = Screen::new();
        let mut read_proxy = screen.read_proxy();

        let read_input = async {
            // send all previous joystick inputs
            for l in InputSnake::new(JOYSTICK_INPUT_FILE).snake() {
                let val = l.parse::<i64>().expect("");
                input_proxy.send(val).await;
            }

            // now play the game, saving our progress
            let mut joystick_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(JOYSTICK_INPUT_FILE)
                .unwrap();
            loop {
                if let Some(key) = read_proxy.read_key() {
                    match key {
                        Input::KeyLeft => {
                            writeln!(joystick_file, "-1").expect("file write");
                            input_proxy.send(-1).await
                        },
                        Input::KeyRight => {
                            writeln!(joystick_file, "1").expect("file write");
                            input_proxy.send(1).await
                        },
                        _ => {
                            writeln!(joystick_file, "0").expect("file write");
                            input_proxy.send(0).await
                        },
                    }
                }

                tokio::time::delay_for(Duration::from_millis(100)).await;
            }
        };

        let populate_screen = async {
            // every three output instructions specify the x position, y position, and tile id
            loop {
                let x = output_proxy.recv().await;
                if x.is_none() {
                    return;
                }

                let x = x.unwrap();
                let y = output_proxy.recv().await.unwrap();
                if x == SCORE_COORD.0 && y == SCORE_COORD.1 {
                    let score = output_proxy.recv().await.unwrap();
                    screen.set_score(score);
                    screen.draw();
                    continue;
                }

                let tile = Tile::from(output_proxy.recv().await.unwrap());
                screen.add_tile((x, y), tile);
                screen.draw();
            }
        };

        runtime.block_on({
            let screen_run = futures::future::join(read_input, populate_screen);
            futures::future::join(self.cpu.run(), screen_run)
        });
        screen
    }
}

impl Screen {
    pub fn new() -> Screen {
        let mut grid = Grid::new();
        grid.init_curses();
        grid.init_curses_input(INPUT_WIN_COORD.0, INPUT_WIN_COORD.1);
        Screen {
            score: 0,
            grid,
        }
    }

    pub fn add_tile(&mut self, coord: Coord, tile: Tile) {
        self.grid.add_entry(coord, tile)
    }

    pub fn tiles(&self) -> Vec<&Tile> {
        self.grid.entries()
    }

    pub fn set_score(&mut self, score: i64) {
        self.score = score;
    }

    pub fn draw(&mut self) {
        let score_str = format!("SCORE = {}", self.score);
        match self.grid.draw_type() {
            DrawType::Curses(window) => {
                window.mvaddstr(PRINT_SCORE_COORD.0, PRINT_SCORE_COORD.1, score_str);
            },
            DrawType::StdOut => {
                println!("{}", score_str);
            },
        }
        self.grid.draw(Screen::draw_tile);
    }

    pub fn read_proxy(&mut self) -> ReadProxy {
        self.grid.read_proxy()
    }

    fn draw_tile(coord: Coord, tile: Option<&Tile>) -> Option<char> {
        if let Some(tile) = tile {
            Some(tile.char())
        } else {
            None
        }
    }
}

fn draw_tile(coord: Coord, tile: Option<&Tile>) -> char {
    if let Some(tile) = tile {
        tile.char()
    } else {
        ' '
    }
}

fn part_one() {
    let input = InputSnake::new("input");
    let mut arcade = Arcade::new(&input.no_snake());

    let screen = arcade.run();
    let num_blocks = screen.tiles()
        .into_iter()
        .filter(|&&tile| tile == Tile::Block)
        .count();

    println!("Part One: {:?}", num_blocks);
}

fn part_two() {
    let input = InputSnake::new("input");
    let mut arcade = Arcade::new(&input.no_snake());

    arcade.free_play();
    let screen = arcade.run();

    let score_per_block = 25;
    let num_blocks = screen.tiles()
        .into_iter()
        .filter(|&&tile| tile == Tile::Block)
        .count();

    println!("Part Two: {:?}", score_per_block * num_blocks);
}

fn main() {
    env_logger::init();

//    part_one();
    part_two();
}
