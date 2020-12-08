use log::{SetLoggerError, LevelFilter};
use log::{debug, info, warn, error};
use env_logger;
use regex::CaptureMatches;

use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

const INPUT_REGEX: &str = r"^(\d+)-(\d+) (\w): (\w+)$";

#[derive(Debug)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    character: char,
}

#[derive(Debug)]
struct Input {
    password_policy: PasswordPolicy,
    password: String,
}

impl PasswordPolicy {
    fn is_valid_password_part1(&self, password: &str) -> bool {
        let count = password.matches(self.character).count();
        return self.min <= count && count <= self.max;
    }

    fn is_valid_password_part2(&self, password: &str) -> bool {
        let char1 = password.chars().nth(self.min - 1).unwrap();
        let char2 = password.chars().nth(self.max - 1).unwrap();
        return (char1 == self.character) ^ (char2 == self.character);
    }
}

impl FromRegex for Input {
    fn from(mut capture_matches: CaptureMatches) -> Self {
        let captures = capture_matches.next().unwrap();
        let password_policy = PasswordPolicy {
            min: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            max: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            character: captures.get(3).unwrap().as_str().chars().next().unwrap(),
        };
        return Input {
            password_policy,
            password: captures.get(4).unwrap().as_str().to_string(),
        }
    }
}


fn part_one() {
    let count = InputSnake::new("input")
        .regex_snake::<Input>(INPUT_REGEX)
        .filter(|i| i.password_policy.is_valid_password_part1(&i.password))
        .count();

    info!("Part One: {:?}", count);
}

fn part_two() {
    let count = InputSnake::new("input")
        .regex_snake::<Input>(INPUT_REGEX)
        .filter(|i| i.password_policy.is_valid_password_part2(&i.password))
        .count();

    info!("Part Two: {:?}", count);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"));
    part_one();
    part_two();
}
