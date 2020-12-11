use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::CaptureMatches;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::cmp::max;

use advent::{InputSnake, FromRegex};

fn part_one() {
    let mut adapters: Vec<i64> = InputSnake::new("input")
        .int_snake()
        .sorted()
        .collect();

    let power_outlet_joltage: i64 = 0;
    let built_in_adapter_joltage: i64 = adapters.last().unwrap() + 3;
    adapters.insert(0, power_outlet_joltage);
    adapters.push(built_in_adapter_joltage);

    let distribution = adapters.iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .sorted()
        .group_by(|&d| d)
        .into_iter()
        .map(|(d, vals)| (d, vals.count()))
        .collect::<HashMap<i64, usize>>();

    info!("Part One: {:?}", distribution.get(&1).unwrap() * distribution.get(&3).unwrap());
}

fn part_two() {
    let mut adapters: Vec<i64> = InputSnake::new("input")
        .int_snake()
        .sorted()
        .collect();

    let power_outlet_joltage: i64 = 0;
    let built_in_adapter_joltage: i64 = adapters.last().unwrap() + 3;
    adapters.insert(0, power_outlet_joltage);
    adapters.push(built_in_adapter_joltage);

    let len = adapters.len();
    let mut arrangements: Vec<u64> = vec![0; len];
    arrangements[len - 1] = 1;
    arrangements[len - 2] = 1;
    arrangements[len - 3] = if adapters[len - 1] - adapters[len - 3] <= 3 { 2 } else { 1 };
    for i in (0..len - 3).rev() {
        debug!("{} {} {} {}", i, arrangements[i], arrangements[i+1], arrangements[i+2]);
        arrangements[i] = arrangements[i + 1];
        if adapters[i + 2] - adapters[i] <= 3 {
            arrangements[i] += arrangements[i + 2];
        }
        if adapters[i + 3] - adapters[i] <= 3 {
            arrangements[i] += arrangements[i + 3];
        }
    }

    debug!("{:?}", adapters);
    debug!("{:?}", arrangements);
    info!("Part Two: {:?}", arrangements[0]);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    part_one();
    part_two();
}
