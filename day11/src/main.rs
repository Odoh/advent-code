use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::CaptureMatches;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::thread::sleep;
use std::time::Duration;

use advent::{InputSnake, FromRegex};
use advent::grid::{Grid};
use advent::grid;

fn applied_changes_part_1(grid: &Grid<char>) -> Vec<(grid::Coord, char)> {
    let mut applied_changes = Vec::new();
    for (coord, seat) in grid.coord_entries() {
        let num_occupied_neighbors = grid.adjacent_entries(coord)
            .iter()
            .filter(|e| ***e == '#')
            .count();
        match seat {
            // seat is empty, if no adjacent occupied seats, becomes occupied
            'L' => {
                if num_occupied_neighbors == 0 {
                    applied_changes.push((coord, '#'));
                }
            },

            // seat is occupied, if four or more seats adjacent are occupied, becomes empty
            '#' => {
                if num_occupied_neighbors >= 4 {
                    applied_changes.push((coord, 'L'));
                }
            },

            // empty floor never changes
            '.' => {},

            _ => panic!("Unhandled seat type"),
        }
    }
    applied_changes
}

fn applied_changes_part_2(grid: &Grid<char>) -> Vec<(grid::Coord, char)> {
    let mut applied_changes = Vec::new();
    for (coord, seat) in grid.coord_entries() {
        let num_occupied_neighbors = grid.all_directional_entries(coord)
            .iter()
            .filter(|directional_entries|
                directional_entries.iter()
                    .filter(|&&&e| e == 'L' || e == '#')
                    .next().map(|&&c| c).unwrap_or('L') == '#')
            .count();
        match seat {
            // seat is empty, if no adjacent occupied seats, becomes occupied
            'L' => {
                if num_occupied_neighbors == 0 {
                    applied_changes.push((coord, '#'));
                }
            },

            // seat is occupied, if four or more seats adjacent are occupied, becomes empty
            '#' => {
                if num_occupied_neighbors >= 5 {
                    applied_changes.push((coord, 'L'));
                }
            },

            // empty floor never changes
            '.' => {},

            _ => panic!("Unhandled seat type"),
        }
    }
    applied_changes
}

fn part_one() {
    let mut grid = InputSnake::new("input").grid_snake();
    grid.init_curses();

    loop {
        grid.draw(grid::DRAW_CHAR);

        let applied_changes = applied_changes_part_1(&grid);
        if applied_changes.is_empty() {
            break;
        }

        for (coord, seat) in applied_changes {
            grid.add_entry(coord, seat);
        }
    }

    let num_occupied = grid.entries()
        .iter()
        .filter(|seat| ***seat == '#')
        .count();

    grid.end_curses();
    info!("Part One: {:?}", num_occupied);
}

fn part_two() {
    let mut grid = InputSnake::new("input").grid_snake();
    loop {
        let applied_changes = applied_changes_part_2(&grid);
        if applied_changes.is_empty() {
            break;
        }

        for (coord, seat) in applied_changes {
            grid.add_entry(coord, seat);
        }
    }

    let num_occupied = grid.entries()
        .iter()
        .filter(|seat| ***seat == '#')
        .count();
    info!("Part Two: {:?}", num_occupied);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    // part_one();
    part_two();
}
