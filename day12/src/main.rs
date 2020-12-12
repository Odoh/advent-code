use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::CaptureMatches;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};
use advent::map::{Location, Direction, Viewpoint, Rotation};

struct Waypoint {
    pub location: Location,
}

struct Ferry {
    pub location: Location,
    pub viewpoint: Viewpoint,
    pub waypoint: Waypoint,
}

impl Ferry {
    pub fn new() -> Self {
        Ferry {
            location: Location::new(),
            viewpoint: Viewpoint::new(Direction::East),
            waypoint: Waypoint {
                location: Location::new_at_location(&[(Direction::East, 10), (Direction::North, 1)])
            }
        }
    }

    pub fn apply_line_part1(&mut self, line: &str) {
        let action = line.chars().next().unwrap();
        let value = line.chars().skip(1).collect::<String>().parse::<i64>().unwrap();

        match action {
            'N'|'S'|'E'|'W' => {
                let direction = Direction::from(action);
                self.location.movement(direction, value);
            },
            'F' => self.location.movement(self.viewpoint.direction(), value),
            'L'|'R' => {
                let rotation = Rotation::from(action);
                self.viewpoint.rotate(rotation, value);
            },
            _ => panic!("Unhandled action: {}", action),
        }
    }

    pub fn apply_line_part2(&mut self, line: &str) {
        let action = line.chars().next().unwrap();
        let value = line.chars().skip(1).collect::<String>().parse::<i64>().unwrap();

        match action {
            'N'|'S'|'E'|'W' => {
                let direction = Direction::from(action);
                self.waypoint.location.movement(direction, value);
            },
            'F' => (0..value).for_each(|_| self.location.move_to_location(&self.waypoint.location)),
            'L'|'R' => {
                let rotation = Rotation::from(action);
                self.waypoint.location.relative_rotate(rotation, value);
            },
            _ => panic!("Unhandled action: {}", action),
        }
    }
}

fn part_one() {
    let mut ferry = Ferry::new();
    debug!("{}", ferry.location);

    InputSnake::new("input")
        .snake()
        .for_each(|line| {
            ferry.apply_line_part1(&line);
            debug!("{}", ferry.location);
        });

    info!("Part One: {:?}", ferry.location.manhattan_distance());
}

fn part_two() {
    let mut ferry = Ferry::new();
    debug!("{} {}", ferry.location, ferry.waypoint.location);

    InputSnake::new("input")
        .snake()
        .for_each(|line| {
            ferry.apply_line_part2(&line);
            debug!("{} {}", ferry.location, ferry.waypoint.location);
        });

    info!("Part Two: {:?}", ferry.location.manhattan_distance());
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"));
    // part_one();
    part_two();
}
