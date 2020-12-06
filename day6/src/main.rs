use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::Captures;

use std::collections::HashSet;
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

fn part_one() {
    let unique_question_count = InputSnake::new("input").group_snake()
        .map(|group|
            group.iter()
                .map(|person| person.chars())
                .flatten()
                .unique()
                .count())
        .sum::<usize>();
    info!("Part One: {:?}", unique_question_count);
}

fn part_two() {
    let common_question_count = InputSnake::new("input").group_snake()
        .map(|group|
            group.iter()
                .map(|person| person.chars().collect::<HashSet<char>>())
                .fold1(|a, b| a.intersection(&b).map(|&c| c).collect::<HashSet<char>>())
                .unwrap_or(HashSet::new())
                .len())
        .sum::<usize>();
    info!("Part Two: {:?}", common_question_count);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"));
    part_one();
    part_two();
}
