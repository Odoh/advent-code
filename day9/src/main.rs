use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use nom::character;
use nom::character::complete::{newline, space1};
use nom::multi::separated_list1;
use regex::CaptureMatches;
use once_cell::sync::Lazy;
use nom::{bytes::complete::*, combinator::*, error::*, sequence::*, IResult, Parser};

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

fn parse_history(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, history) = separated_list1(
        space1,
        character::complete::i32,
    )(input)?;

    Ok((
        input,
        history,
    ))
}

fn extrapolate(history: &Vec<i32>) -> i32 {
    if history.iter().all(|&h| h == 0) {
        return 0;
    }

    let last = history.last().unwrap();
    let diffs = history.iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    debug!("{:?} {:?}", last, diffs);
    return last + extrapolate(&diffs);
}

/// ------
/// Part 1
/// ------

fn part_one_test() {
    let input = InputSnake::new("test_input");
    let histories = input.nom_snake(parse_history)
        .map(|mut o| o.parse())
        .collect_vec();
    debug!("{:?}", histories);

    let extrapolated = histories.iter()
        .map(|history| extrapolate(history))
        .collect_vec();
    debug!("{:?}", extrapolated);

    let sum_extrapolated: i32 = extrapolated.iter().sum();

    info!("{:?}", sum_extrapolated);
}

fn part_one() {
    let input = InputSnake::new("input");
    let histories = input.nom_snake(parse_history)
        .map(|mut o| o.parse())
        .collect_vec();

    let extrapolated = histories.iter()
        .map(|history| extrapolate(history))
        .collect_vec();

    let sum_extrapolated: i32 = extrapolated.iter().sum();
    info!("{:?}", sum_extrapolated);
}

/// ------
/// Part 2
/// ------

fn part_two_test() {
    let input = InputSnake::new("test_input");
    let histories = input.nom_snake(parse_history)
        .map(|mut o| {
            let mut values = o.parse();
            values.reverse();
            values
        })
        .collect_vec();

    let extrapolated = histories.iter()
        .map(|history| extrapolate(history))
        .collect_vec();

    let sum_extrapolated: i32 = extrapolated.iter().sum();

    info!("{:?}", sum_extrapolated);
}

fn part_two() {
    let input = InputSnake::new("input");
    let histories = input.nom_snake(parse_history)
        .map(|mut o| {
            let mut values = o.parse();
            values.reverse();
            values
        })
        .collect_vec();

    let extrapolated = histories.iter()
        .map(|history| extrapolate(history))
        .collect_vec();

    let sum_extrapolated: i32 = extrapolated.iter().sum();

    info!("{:?}", sum_extrapolated);
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
