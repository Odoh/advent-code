use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::CaptureMatches;
use once_cell::sync::Lazy;
use nom::{bytes::complete::*, combinator::*, error::*, sequence::*, IResult, Parser};

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

/// ------
/// Part 1
/// ------

fn part_one_test() {
    let input = InputSnake::new("test_input");
    info!("{:?}", 1);
}

fn part_one() {
    let input = InputSnake::new("input");
    info!("{:?}", 1);
}

/// ------
/// Part 2
/// ------

fn part_two_test() {
    let input = InputSnake::new("test_input");
    info!("{:?}", 2);
}

fn part_two() {
    let input = InputSnake::new("input");
    info!("{:?}", 2);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"));

    info!("Part One Test");
    part_one_test();
    info!("Part One");
    part_one();
 
    info!("Part Two Test");
    part_two_test();
    info!("Part Two");
    part_two();
}
