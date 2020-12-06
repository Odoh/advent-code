use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::Captures;

use std::collections::HashSet;
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

fn part_one() {
    let input = InputSnake::new("input");
    info!("Part One: {:?}", 1);
}

fn part_two() {
    let input = InputSnake::new("input");
    info!("Part Two: {:?}", 2);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"));
    part_one();
    part_two();
}
