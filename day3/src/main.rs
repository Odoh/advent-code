use advent::grid::Coord;
use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::CaptureMatches;
use once_cell::sync::Lazy;

use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt::Debug;

use advent::{InputSnake, FromRegex, grid::Grid};

/// ------
/// Part 1
/// ------

fn is_symbol(c: char) -> bool {
    if c.is_digit(10) {
        return false;
    }

    if c == '.' {
        return false;
    }

    true
}

fn find_part_numbers(grid: &mut Grid<char>) -> Vec<u32> {
    // iterate through all entries in the grid, building up the list of part numbers.
    // This requires maintaining state for:
    // 1. adjacent numbers (one after another) to build the final number
    // 2. whether any digit in the part number is adjacent to a symbol
    let mut part_numbers = Vec::new();

    let mut number = String::new();
    let mut is_part_number = false;
    for (coord, &c) in grid.iter().coords() {
        // if character is not a digit, save the last known number (if needed) and then continue 
        if !c.is_digit(10) {
            if !number.is_empty() {
                if is_part_number {
                    let part_number = number.parse::<u32>().expect("A valid number");
                    part_numbers.push(part_number);

                    is_part_number = false;
                }
                number.clear();
            }

            continue;
        }

        // character is a digit, update the part number state
        number.push(c);

        if grid.adjacent_entries(coord).into_iter().any(|&a| is_symbol(a)) {
            is_part_number = true;
        }
    }

    part_numbers
}

fn part_one_test() {
    let input = InputSnake::new("test_input");
    let mut grid = input.grid_snake();

    let part_numbers = find_part_numbers(&mut grid);
    // dbg!(part_numbers);

    info!("Part nubmer sum: {:?}", part_numbers.iter().sum::<u32>());
}

fn part_one() {
    let input = InputSnake::new("input");
    let mut grid = input.grid_snake();

    let part_numbers = find_part_numbers(&mut grid);
    info!("Part nubmer sum: {:?}", part_numbers.iter().sum::<u32>());
}

/// ------
/// Part 2
/// ------
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct PartNumber {
    coord: Coord,
    number: u32,
}

#[derive(Debug)]
struct Gear {
    coord: Coord,
    part_numbers: (PartNumber, PartNumber),
}

impl Gear {
    pub fn gear_ratio(&self) -> u32 {
        self.part_numbers.0.number * self.part_numbers.1.number
    }
}

fn find_gears(grid: &mut Grid<char>) -> Vec<Gear> {
    // iterate through all entries in the grid, building up the list of gears.
    // This is the same logic as for finding part numbers, except:
    // 1. when building a part number also store the adjacent gears
    let mut gears_by_part_number: HashMap<PartNumber, HashSet<Coord>> = HashMap::new();

    let mut iter_number = String::new();
    let mut adjacent_gears: HashSet<Coord> = HashSet::new();
    for (coord, &c) in grid.iter().coords() {
        // if character is not a digit, save the last known number (if needed) and then continue 
        if !c.is_digit(10) {
            if !iter_number.is_empty() {
                if !adjacent_gears.is_empty() {
                    let number = iter_number.parse::<u32>().expect("A valid number");
                    let part_number = PartNumber {
                        coord,
                        number,
                    };
                    gears_by_part_number.insert(part_number, adjacent_gears);

                    adjacent_gears = HashSet::new();
                }
                iter_number.clear();
            }

            continue;
        }

        // character is a digit, update the part number state
        iter_number.push(c);

        grid.adjacent_coord_entries(coord).into_iter()
            .filter(|(_, &c)| c == '*')
            .for_each(|(coord, _)| { adjacent_gears.insert(coord); });
    }

    // reverse the map, populating the gears adjancent to each part number
    let part_numbers_by_gear: HashMap<Coord, Vec<PartNumber>> = gears_by_part_number.into_iter()
        .flat_map(|(k, vs)| vs.into_iter().map(move |v| (v, k)))
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_default().push(v);
            acc
        });

    // populate the gear struct with the actual gears (only consist of two part numbers)
    let gears = part_numbers_by_gear.into_iter()
        .filter(|(_coord, part_nos)| part_nos.len() == 2)
        .map(|(coord, part_nos)| Gear {
            coord,
            part_numbers: (*part_nos.get(0).unwrap(), *part_nos.get(1).unwrap())
        })
        .collect();

    // dbg!(gears);
    // todo!();

    gears
}

fn part_two_test() {
    let input = InputSnake::new("test_input");
    let mut grid = input.grid_snake();

    let gears = find_gears(&mut grid);
    info!("Gear ratio sum: {:?}", gears.iter().map(|g| g.gear_ratio()).sum::<u32>());
}

fn part_two() {
    let input = InputSnake::new("input");
    let mut grid = input.grid_snake();

    let gears = find_gears(&mut grid);
    info!("Gear ratio sum: {:?}", gears.iter().map(|g| g.gear_ratio()).sum::<u32>());
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

    info!("Part One Test");
    part_one_test();
    info!("Part One");
    part_one();
 
    info!("Part Two Test");
    part_two_test();
    info!("Part Two");
    part_two();
}
