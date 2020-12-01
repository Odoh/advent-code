use log::{SetLoggerError, LevelFilter};
use log::{debug, info, warn, error};
use env_logger;

use advent::InputSnake;

const SOLUTION_SUM: i64 = 2020;

fn part_one() {
    let input = InputSnake::new("input");
    for i in input.int_snake() {
        for j in input.int_snake() {
            if (i + j) == SOLUTION_SUM {
                info!("Part One: {:?}", i * j);
                return;
            }
        }
    }
}

fn part_two() {
    let input = InputSnake::new("input");
    for i in input.int_snake() {
        for j in input.int_snake() {
            for k in input.int_snake() {
                if (i + j + k) == SOLUTION_SUM {
                    info!("Part Two: {:?}", i * j * k);
                    return;
                }
            }
        }
    }
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"));
    part_one();
    part_two();
}
