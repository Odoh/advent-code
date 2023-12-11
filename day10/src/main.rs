use advent::grid::{Grid, Coord, Direction, CARDINAL_DIRECTIONS};
use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger::{self, filter};
use regex::CaptureMatches;
use once_cell::sync::Lazy;
use nom::{bytes::complete::*, combinator::*, error::*, sequence::*, IResult, Parser};

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
const PIPE_ENTER: Lazy<HashMap<char, Vec<Direction>>> = Lazy::new(|| {
    vec!(
        ('|', vec!(Direction::Down, Direction::Up)),
        ('-', vec!(Direction::Right, Direction::Left)),
        ('L', vec!(Direction::Down, Direction::Left)),
        ('J', vec!(Direction::Down, Direction::Right)),
        ('7', vec!(Direction::Up, Direction::Right)),
        ('F', vec!(Direction::Up, Direction::Left)),
        ('.', vec!()),
        ('S', vec!(Direction::Up, Direction::Down, Direction::Left, Direction::Right)),
    ).into_iter()
    .collect()
});
const PIPE_EXIT: Lazy<HashMap<char, Vec<Direction>>> = Lazy::new(|| {
    vec!(
        ('|', vec!(Direction::Down, Direction::Up)),
        ('-', vec!(Direction::Right, Direction::Left)),
        ('L', vec!(Direction::Up, Direction::Right)),
        ('J', vec!(Direction::Up, Direction::Left)),
        ('7', vec!(Direction::Down, Direction::Left)),
        ('F', vec!(Direction::Down, Direction::Right)),
        ('.', vec!()),
        ('S', vec!(Direction::Up, Direction::Down, Direction::Left, Direction::Right)),
    ).into_iter()
    .collect()
});

fn get_next_directions(grid: &Grid<char>, directions: &[Direction], coord: Coord) -> Vec<(Direction, Coord)> {
    let entry = grid.entry(coord).unwrap();
    let next_directions = directions.iter()
        .filter_map(|&direction| {
            let (next_coord, next_entry) = grid.next_location(coord, direction);
            if next_entry.is_none() {
                return None;
            }

            debug!("{:?} {:?} {:?}", direction, next_coord, next_entry);

            let pipe_enter = PIPE_ENTER;
            let pipe_exit = PIPE_EXIT;
            let coord_directions = pipe_exit.get(entry).unwrap();
            let next_coord_directions = pipe_enter.get(next_entry.unwrap()).unwrap();
            if coord_directions.contains(&direction) && next_coord_directions.contains(&direction) {
                return Some((direction, next_coord));
            }

            return None;
        })
        .collect_vec();

    next_directions
}

/// ------
/// Part 1
/// ------

fn find_steps(grid: &Grid<char>) -> u32 {
    // find the start space traverse the grid, then pick one of the two directions to begin the loop
    let start = grid.position('S');
    let start_directions = get_next_directions(grid, &CARDINAL_DIRECTIONS[..], start);
    debug_assert_eq!(start_directions.len(), 2);
    let (start_direction, next_coord) = start_directions.get(0).unwrap();

    let mut steps: u32 = 1;
    let mut coord = *next_coord;
    let mut last_direction = *start_direction;
    loop {
        debug!("Walk: {:?} {:?} {:?}", steps, coord, last_direction);

        let forward_directions = CARDINAL_DIRECTIONS.iter()
            .filter(|&d| *d != last_direction.get_opposite())
            .map(|d| *d)
            .collect_vec();
        let next_directions = get_next_directions(grid, &forward_directions[..], coord);
        // debug!("{:?} {:?}", forward_directions, next_directions);

        debug_assert_eq!(next_directions.len(), 1);
        let (next_direction, next_coord) = next_directions.get(0).unwrap();

        steps += 1;
        coord = *next_coord;
        last_direction = *next_direction;

        if coord == start {
            return steps;
        }
    }
}

fn part_one_test() {
    let input = InputSnake::new("test_input");
    let mut grid = input.grid_snake();
    grid.draw(|coord, val_opt| val_opt.map(|c| *c));

    let steps = find_steps(&grid);

    info!("{:?}", steps / 2);
}

fn part_one_test_two() {
    let input = InputSnake::new("test_input_2");
    let mut grid = input.grid_snake();
    grid.draw(|coord, val_opt| val_opt.map(|c| *c));

    let steps = find_steps(&grid);

    info!("{:?}", steps / 2);
}

fn part_one() {
    let input = InputSnake::new("input");
    let grid = input.grid_snake();

    let steps = find_steps(&grid);
    info!("{:?}", steps / 2);
}

/// ------
/// Part 2
/// ------

fn find_path(grid: &Grid<char>) -> Vec<Coord> {
    // find the start space traverse the grid, then pick one of the two directions to begin the loop
    let start = grid.position('S');
    let start_directions = get_next_directions(grid, &CARDINAL_DIRECTIONS[..], start);
    debug_assert_eq!(start_directions.len(), 2);
    let (start_direction, next_coord) = start_directions.get(0).unwrap();

    let mut path = vec!(start);
    let mut coord = *next_coord;
    let mut last_direction = *start_direction;
    loop {
        let forward_directions = CARDINAL_DIRECTIONS.iter()
            .filter(|&d| *d != last_direction.get_opposite())
            .map(|d| *d)
            .collect_vec();
        let next_directions = get_next_directions(grid, &forward_directions[..], coord);
        // debug!("{:?} {:?}", forward_directions, next_directions);

        debug_assert_eq!(next_directions.len(), 1);
        let (next_direction, next_coord) = next_directions.get(0).unwrap();

        path.push(coord);
        coord = *next_coord;
        last_direction = *next_direction;

        if coord == start {
            return path;
        }
    }
}

fn part_two_test() {
    let input = InputSnake::new("test_input");
    let mut grid = input.grid_snake();
    let path = find_path(&grid);

    grid.draw(|coord, val_opt| if path.contains(&coord) { val_opt.map(|c| *c) } else { Some('.') });
    // grid.draw(|coord, val_opt| if path.contains(&coord) {
    //     val_opt.map(|c| *c)
    // } else {
    //         None
    // });

    info!("{:?}", 2);
}

fn part_two() {
    let input = InputSnake::new("input");
    info!("{:?}", 2);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

    // info!("Part One Test");
    // part_one_test();
    // info!("Part One");
    // part_one();
 
    info!("Part Two Test");
    part_two_test();
    info!("Part Two");
    part_two();
}
