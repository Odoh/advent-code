use itertools::Itertools;
use log::{debug, info, warn, error};
use env_logger;
use nom::bytes::complete::tag;
use nom::{IResult, character};
use nom::character::complete::{space1, newline};
use nom::multi::separated_list1;
use nom::sequence::{preceded, pair};
use regex::CaptureMatches;
use once_cell::sync::Lazy;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::iter::zip;

use advent::{InputSnake, FromRegex};

// Time:      7  15   30
// Distance:  9  40  200
#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

/// ------
/// Part 1
/// ------

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = preceded(
        pair(tag("Time: "), space1),
        separated_list1(
            space1,
            character::complete::u64,
        )
    )(input)?;

    let (input, _) = newline(input)?;

    let (input, distances) = preceded(
        pair(tag("Distance: "), space1),
        separated_list1(
            space1,
            character::complete::u64,
        )
    )(input)?;

    debug_assert_eq!(times.len(), distances.len());

    let races = zip(times, distances)
        .map(|(time, distance)| Race {
            time,
            distance,
        })
        .collect();

    Ok((
        input,
        races
    ))
}


fn part_one_test() {
    let input = InputSnake::new("test_input");
    let no_snake = input.no_snake();
    let (_, races) = parse_races(&no_snake).unwrap();

    let margin_of_error: u64 = races.into_iter()
        .map(|race| (0..race.time)
            .map(|charge_time| charge_time * (race.time - charge_time))
            .filter(|&distance| distance > race.distance)
            .count() as u64)
        .product();

    info!("{:?}", margin_of_error);
}

fn part_one() {
    let input = InputSnake::new("input");
    let no_snake = input.no_snake();
    let (_, races) = parse_races(&no_snake).unwrap();

    let margin_of_error: u64 = races.into_iter()
        .map(|race| (0..race.time)
            .map(|charge_time| charge_time * (race.time - charge_time))
            .filter(|&distance| distance > race.distance)
            .count() as u64)
        .product();

    info!("{:?}", margin_of_error);
}

/// ------
/// Part 2
/// ------

fn parse_races_two(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = preceded(
        pair(tag("Time: "), space1),
        separated_list1(
            space1,
            character::complete::u64,
        )
    )(input)?;

    let (input, _) = newline(input)?;

    let (input, distances) = preceded(
        pair(tag("Distance: "), space1),
        separated_list1(
            space1,
            character::complete::u64,
        )
    )(input)?;

    debug_assert_eq!(times.len(), distances.len());

    let time = times.into_iter()
        .map(|time| format!("{time}"))
        .join("")
        .parse::<u64>().expect("valid time");
    let distance = distances.into_iter()
        .map(|distance| format!("{distance}"))
        .join("")
        .parse::<u64>().expect("valid distance");

    let races = vec!(Race { time, distance });

    Ok((
        input,
        races
    ))
}

fn part_two_test() {
    let input = InputSnake::new("test_input");
    let no_snake = input.no_snake();
    let (_, races) = parse_races_two(&no_snake).unwrap();

    let margin_of_error: u64 = races.into_iter()
        .map(|race| (0..race.time)
            .map(|charge_time| charge_time * (race.time - charge_time))
            .filter(|&distance| distance > race.distance)
            .count() as u64)
        .product();

    info!("{:?}", margin_of_error);
}

fn part_two() {
    let input = InputSnake::new("input");
    let no_snake = input.no_snake();
    let (_, races) = parse_races_two(&no_snake).unwrap();

    let margin_of_error: u64 = races.into_iter()
        .map(|race| (0..race.time)
            .map(|charge_time| charge_time * (race.time - charge_time))
            .filter(|&distance| distance > race.distance)
            .count() as u64)
        .product();

    info!("{:?}", margin_of_error);
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
