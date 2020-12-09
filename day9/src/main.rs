use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::CaptureMatches;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

const PREAMBLE_LEN: usize = 25;

fn part_one() {
    let input = InputSnake::new("input")
        .int_snake()
        .collect::<Vec<i64>>();

    let invalid = input.iter()
        .enumerate()
        .skip(PREAMBLE_LEN)
        .filter(|&(i, &v)| {
            let prev_slice1 = &input[(i - PREAMBLE_LEN)..i];
            let prev_slice2 = &input[(i - PREAMBLE_LEN)..i];
            !prev_slice1.iter().cartesian_product(prev_slice2.iter())
                .map(|(x, y)| x + y)
                .any(|sum| sum == v)
        })
        .map(|(_, &v)| v)
        .collect::<Vec<i64>>();

    info!("Part One: {:?}", invalid.first().unwrap());
}

fn part_two() {
    let input = InputSnake::new("input")
        .int_snake()
        .collect::<Vec<i64>>();

    let invalid_number: i64 = 1930745883;
    for i in 0..input.len() {
        let mut sum: Vec<i64> = Vec::new();
        for j in i..input.len() {
            if sum.iter().sum::<i64>() == invalid_number {
                let min = sum.iter().min().unwrap();
                let max = sum.iter().max().unwrap();
                info!("Part Two: {:?}", min + max);
                return;
            }

            sum.push(input[j]);
        }
    }
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"));
    part_one();
    part_two();
}
