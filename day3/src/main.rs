use log::{debug, info, warn, error};
use env_logger;
use regex::Captures;

use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

const TREE: char = '#';

struct Slope {
    right: usize,
    down: usize,
}

fn num_trees(input: &str, slopes: &[Slope]) -> Vec<usize> {
    slopes.iter()
        .map(|s| {
            InputSnake::new(input)
                .snake()
                .enumerate()
                .step_by(s.down)
                .filter(|(i, line)| {
                    let j = (i / s.down) * s.right;
                    let char = line.chars().cycle().nth(j).unwrap();
                    debug!("{},{} {}", i, j, char);

                    char == TREE
                }).count()
        })
        .collect()
}

fn part_one() {
    let slopes: [Slope; 1] = [
        Slope { right: 3, down: 1, }
    ];
    let trees: usize = num_trees("input", &slopes[..])
        .iter()
        .sum();
    info!("Part One: {:?}", trees);
}

fn part_two() {
    let slopes: [Slope; 5] = [
        Slope { right: 1, down: 1, },
        Slope { right: 3, down: 1, },
        Slope { right: 5, down: 1, },
        Slope { right: 7, down: 1, },
        Slope { right: 1, down: 2, },
    ];
    let trees = num_trees("input", &slopes[..]);
    let trees_multiplied: usize = trees.iter().product();
    info!("Part Two: {:?}", trees_multiplied);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    part_one();
    part_two();
}
