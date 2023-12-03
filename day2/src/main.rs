use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::CaptureMatches;
use once_cell::sync::Lazy;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn into_tuple<T, I>(mut split: I) -> (T, T)
where
    I: Iterator<Item = T> {

    let one = split.next().expect("Iterator has one element");
    let two = split.next().expect("Iterator has two elements");
    (one, two)
}

fn parse_games<T, I>(snake: I) -> Vec<Game>
where
    T: AsRef<str> + std::fmt::Display,
    I: Iterator<Item = T> {

    snake.map(|line| {
        let mut split = line.as_ref().split(':');
        let (game, sets) = into_tuple(split);

        let game_split = game.split_whitespace();
        let (_, game_id) = into_tuple(game_split);
        let id: u32 = game_id.parse::<u32>().expect("Game number is a number");

        let sets: Vec<Set> = sets.split(';')
            .map(|set| {
                let colors: HashMap<&str, u32> = set.split(',')
                    .map(|color| {
                        let color_split = color.trim().split_whitespace();
                        let (number, color) = into_tuple(color_split);
                        (color, number.parse::<u32>().expect("Color number is a number"))
                    })
                    .collect();
                Set {
                    red: colors.get("red").map(|&v| v).unwrap_or(0),
                    green: colors.get("green").map(|&v| v).unwrap_or(0),
                    blue: colors.get("blue").map(|&v| v).unwrap_or(0),
                }
            })
            .collect();

        Game {
            id,
            sets,
        }
    })
    .collect()
}

/// ------
/// Part 1
/// ------

fn part_one_test() {
    let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    let games = parse_games(input.lines().skip(1));
    // dbg!(games);

    const RED_CUBES: u32 = 12;
    const GREEN_CUBES: u32 = 13;
    const BLUE_CUBES: u32 = 14;

    let game_id_sum: u32 = games.iter()
        .filter(|game| game.sets.iter()
            .all(|set| set.red <= RED_CUBES && set.green <= GREEN_CUBES && set.blue <= BLUE_CUBES))
        .map(|game| game.id)
        .sum();

     info!("{:?}", game_id_sum);
}

fn part_one() {
    let input = InputSnake::new("input");

    const RED_CUBES: u32 = 12;
    const GREEN_CUBES: u32 = 13;
    const BLUE_CUBES: u32 = 14;

    let games = parse_games(input.snake());
    let game_id_sum: u32 = games.iter()
        .filter(|game| game.sets.iter()
            .all(|set| set.red <= RED_CUBES && set.green <= GREEN_CUBES && set.blue <= BLUE_CUBES))
        .map(|game| game.id)
        .sum();

     info!("{:?}", game_id_sum);
}

/// ------
/// Part 2
/// ------

fn part_two_test() {
    let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    let games = parse_games(input.lines().skip(1));

    let cube_power_sum: u32 = games.iter()
        .map(|game| {
            let max_red = game.sets.iter().map(|set| set.red).max().expect("red to exist in set");
            let max_green = game.sets.iter().map(|set| set.green).max().expect("green to exist in set");
            let max_blue = game.sets.iter().map(|set| set.blue).max().expect("blue to exist in set");
            max_red * max_green * max_blue
        })
        .sum();

     info!("{:?}", cube_power_sum);
}

fn part_two() {
    let input = InputSnake::new("input");

    let games = parse_games(input.snake());
    let cube_power_sum: u32 = games.iter()
        .map(|game| {
            let max_red = game.sets.iter().map(|set| set.red).max().expect("red to exist in set");
            let max_green = game.sets.iter().map(|set| set.green).max().expect("green to exist in set");
            let max_blue = game.sets.iter().map(|set| set.blue).max().expect("blue to exist in set");
            max_red * max_green * max_blue
        })
        .sum();

     info!("{:?}", cube_power_sum);
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
