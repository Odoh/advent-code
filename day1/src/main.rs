use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::CaptureMatches;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use once_cell::sync::Lazy;

use advent::{InputSnake, FromRegex};

/// ------
/// Part 1
/// ------

fn find_calibration<'a, I, T>(snake: I) -> u32
where
    I: Iterator<Item = T>,
    T: AsRef<str> {

    let total_calibration = snake.map(|s| {
        let digits: Vec<u32> = s.as_ref().chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        let combined_digit: u32 = format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse().unwrap();
        combined_digit
    }).sum();

    total_calibration
}

fn part_one_test() {
    let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

    info!("sum calibration: {}", find_calibration(input.split_whitespace()));
}

fn part_one() {
    let input = InputSnake::new("input");
    let digits = input.snake();
    info!("Part One: {:?}", find_calibration(digits));
}

/// ------
/// Part 2
/// ------

const DIGITS: Lazy<Vec<(&'static str, &'static str)>> = Lazy::new(|| {
    vec!(
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    )
});

fn replace_text_digits(text: &str) -> String {
    if text.is_empty() {
        return text.to_owned();
    }

    for (digit_text, digit_val) in DIGITS.iter() {
        if text.starts_with(digit_text) {
            let remaining_text = &text[(digit_text.len()-1)..];
            return format!("{}{}", digit_val, replace_text_digits(remaining_text));
        }
    }

    return format!("{}{}", &text[..1], replace_text_digits(&text[1..]));
}

fn find_calibration_two<'a, I, T>(snake: I) -> u32
where
    I: Iterator<Item = T>,
    T: AsRef<str> {

    let total_calibration = snake.map(|s| {
        let digits: Vec<u32> = replace_text_digits(s.as_ref())
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        let combined_digit: u32 = format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse().unwrap();
        combined_digit
    }).sum();

    total_calibration
}

fn part_two_test() {
    let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

    info!("sum calibration: {}", find_calibration_two(input.split_whitespace()));
}

fn part_two() {
    let input = InputSnake::new("input");
    let digits = input.snake();
    info!("Part Two: {:?}", find_calibration_two(digits));
}

// ----
// Main
// ----

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"));

    info!("Part 1 Test");
    part_one_test();
    info!("Part 1");
    part_one();
 
    info!("Part 2 Test");
    part_two_test();
    info!("Part 2");
    part_two();
}
