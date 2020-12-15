use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::CaptureMatches;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

#[derive(Debug)]
struct Input {
    depart_time: u64,
    bus_ids: Vec<u64>,
}

impl Input {
    pub fn from(input_snake: InputSnake) -> Input {
        let mut snake = input_snake.snake();
        let depart_time = snake.next().unwrap().parse::<u64>().unwrap();
        let bus_ids = snake.next().unwrap().split(',')
            .filter(|&line| line != "x")
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Input {
            depart_time,
            bus_ids,
        }
    }

    pub fn earliest_bus_for_depart_time(&self) -> (u64, u64) {
        let bus_routes = self.bus_routes_until_time(self.depart_time);
        let soonest_bus_route = bus_routes.into_iter()
            .map(|(bus_id, route)| (bus_id, route.last().unwrap()))
            .min_by_key(|(bus_id, last_time)| *last_time)
            .map(|(bus_id, last_time)| (bus_id, last_time))
            .unwrap();
        soonest_bus_route
    }

    fn bus_routes_until_time(&self, time: u64) -> HashMap<u64, Box<impl Iterator<Item = u64>>> {
        let mut bus_routes = HashMap::new();
        self.bus_ids.iter()
            .for_each(|&bus_id| {
                let bus_route = Box::new((0..=(time + bus_id)).step_by(bus_id as usize));
                bus_routes.insert(bus_id, bus_route);
            });
        bus_routes
    }
}

fn part_one() {
    let input = Input::from(InputSnake::new("input"));
    let (bus_id, earliest_time) = input.earliest_bus_for_depart_time();
    let value = (earliest_time - input.depart_time) * bus_id;
    info!("Part One: {:?}", value);
}

fn part_two() {
    let input = InputSnake::new("input");
    for i in (0..100000000000000u64).step_by(43) {
        debug!("test");
    }
    info!("Part Two: {:?}", 2);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    // part_one();
    part_two();
}
