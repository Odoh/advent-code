use std::ops::RangeInclusive;
use std::collections::HashSet;
use log::{debug, info};
use itertools::Itertools;
use env_logger;

use std::fmt::Debug;

use advent::InputSnake;

const PLANE_ROWS: u64 = 127;
const PLANE_COLS: u64 = 7;

trait BinaryPartition {
    fn mid(&self) -> u64;
}

impl BinaryPartition for RangeInclusive<u64> {
    fn mid(&self) -> u64 {
        (self.start() + self.end() + 1) / 2
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Seat {
    id: u64,
    row: u64,
    col: u64,
}

impl Seat {
    fn new(coord: (u64, u64)) -> Self {
        let row = coord.0;
        let col = coord.1;
        Seat {
            id: Seat::id(row, col),
            row,
            col,
        }
    }

    fn from(input: &str) -> Self {
        let mut rows = 0 ..= PLANE_ROWS;
        let mut cols = 0 ..= PLANE_COLS;
        for c in input.chars() {
            match c {
                'F' => rows = *rows.start() ..= rows.mid(),
                'B' => rows = rows.mid() ..= *rows.end(),
                'L' => cols = *cols.start() ..= cols.mid(),
                'R' => cols = cols.mid() ..= *cols.end(),
                _ => panic!("Unhandled character {}", c),
            }
        }

        let row = *rows.start();
        let col = *cols.start();
        Seat {
            id: Seat::id(row, col),
            row,
            col,
        }
    }

    fn id(row: u64, col: u64) -> u64 {
        row * 8 + col
    }
}

fn part_one() {
    let seats = InputSnake::new("input")
        .snake()
        .map(|line| Seat::from(&line))
        .collect::<Vec<Seat>>();

    debug!("{:?}", seats);

    let max_seat_id = seats.iter()
        .map(|seat| seat.id)
        .max()
        .unwrap();
    info!("Part One: {:?}", max_seat_id);
}

fn part_two() {
    let seats = InputSnake::new("input")
        .snake()
        .map(|line| Seat::from(&line))
        .collect::<HashSet<Seat>>();
    let seat_ids = seats.iter()
        .map(|seat| seat.id)
        .collect::<HashSet<u64>>();
    
    let all_seats = (0 ..= PLANE_ROWS)
        .cartesian_product(0 ..= PLANE_COLS)
        .map(|coord| Seat::new(coord))
        .collect::<HashSet<Seat>>();

    // seat is not in the front or back, and neighbor seat ids exist
    let candidate_seats = all_seats.difference(&seats)
        .filter(|&seat| seat.row != 0 && seat.row != PLANE_ROWS) 
        .filter(|&seat| seat_ids.contains(&(seat.id + 1)) && seat_ids.contains(&(seat.id - 1))) 
        .collect::<Vec<&Seat>>();

    debug!("{:?}", candidate_seats);

    let seat = candidate_seats.first().unwrap();
    info!("Part Two: {:?}", seat.id);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    part_one();
    part_two();
}
