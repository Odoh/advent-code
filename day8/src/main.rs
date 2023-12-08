use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use nom::character;
use nom::character::complete::{alpha1, space1, newline, space0, alphanumeric1};
use nom::multi::{many0, separated_list1, fold_many1};
use regex::CaptureMatches;
use once_cell::sync::Lazy;
use nom::{bytes::complete::*, combinator::*, error::*, sequence::*, IResult, Parser};

use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt::Debug;
use std::mem;
use std::rc::Rc;

use advent::{InputSnake, FromRegex};

#[derive(Debug)]
struct Document<'a> {
    instructions: &'a str,
    nodes: BTreeMap<&'a str, (&'a str, &'a str)>
}

fn parse_document(input: &str) -> IResult<&str, Document> {
    let (input, instructions) = alphanumeric1(input)?;
    let (input, _) = newline(input)?;

    let (input, nodes) = fold_many1(
        preceded(
            newline,
            separated_pair(
                alphanumeric1,
                tuple((space1, tag("="), space1)),
                separated_pair(
                    preceded(tag("("), alphanumeric1),
                    tuple((tag(","), space1)),
                    terminated(alphanumeric1, tag(")"))
                )
            )
        ),
        BTreeMap::new,
        |mut acc, (parent, (left, right))| {
            acc.insert(parent, (left, right));
            acc
        }
    )(input)?;

    Ok((
        input,
        Document {
            instructions,
            nodes,
        }
    ))
}

/// ------
/// Part 1
/// ------

fn num_steps(document: &Document) -> u32 {
    let mut steps = 0;
    let mut location = "AAA";
    loop {
        for dir in document.instructions.chars().cycle() {
            let node = document.nodes.get(location).unwrap();
            location = match dir {
                'L' => node.0,
                'R' => node.1,
                _ => panic!("Unknown character"),
            };
            steps += 1;

            if location == "ZZZ" {
                return steps;
            }
        }
    }
}

fn part_one_test() {
    let input = InputSnake::new("test_input");
    let no_snake = input.no_snake();
    let (_, document) = parse_document(&no_snake).unwrap();
    dbg!(&document);

    let num_steps = num_steps(&document);
    info!("{:?}", num_steps);
}

fn part_one_test_two() {
    let input = InputSnake::new("test_input_2");
    let no_snake = input.no_snake();
    let (_, document) = parse_document(&no_snake).unwrap();

    let num_steps = num_steps(&document);
    info!("{:?}", num_steps);
}

fn part_one() {
    let input = InputSnake::new("input");
    let no_snake = input.no_snake();
    let (_, document) = parse_document(&no_snake).unwrap();

    let num_steps = num_steps(&document);
    info!("{:?}", num_steps);
}

/// ------
/// Part 2
/// ------

type StepCycle<'a> = (&'a str, usize);

#[derive(Debug)]
struct StepCycleData<'a> {
    step_cycle: StepCycle<'a>,
    num_steps: u32,
    term_nodes: Vec<(&'a str, usize)>,
}

fn find_step_cycle<'a>(document: &'a Document, location: &'a str) -> StepCycle<'a> {
    // find the step cycle: when a past location has been seen and we're at the same instruction
    let mut step_history: HashSet<(&str, usize)> = HashSet::new();

    let mut location: &str = location;
    loop {
        for (i, dir) in document.instructions.chars().enumerate().cycle() {
            // debug!("{} {}", i, dir);
            let node = document.nodes.get(location).unwrap();
            location = match dir {
                'L' => node.0,
                'R' => node.1,
                _ => panic!("Unknown character"),
            };

            let history = (location, i);
            // debug!("history {:?}", history);

            if step_history.contains(&history) {
                // the index of loop (the next instruction to execute) is actually one after the
                // current (location is updated above but `i` still has the previous value)
                let instruction = if i + 1 < document.instructions.len() {
                    i + 1
                } else {
                    0
                };
                return (location, instruction);
            }

            step_history.insert(history);
        }
    }
}

fn get_step_cycle_data<'a>(document: &'a Document, step_cycle: StepCycle<'a>) -> StepCycleData<'a> {
    // given a step cycle, gather data about the cycle useful for walking the map as a ghost
    let mut location: &str = step_cycle.0;
    let mut num_steps: u32 = 0;
    let mut term_nodes: Vec<(&'a str, usize)> = Vec::new();

    loop {
        for (i, dir) in document.instructions.chars().enumerate().cycle().skip(step_cycle.1) {
            // debug!("{:?} {:?} {:?}", location, i, dir);
            let node = document.nodes.get(location).unwrap();
            location = match dir {
                'L' => node.0,
                'R' => node.1,
                _ => panic!("Unknown character"),
            };

            if location.ends_with("Z") {
                term_nodes.push((location, i));
            }

            num_steps += 1;

            // debug!("{:?} {:?} == {:?}", location, i, step_cycle);
            let instruction = if i + 1 < document.instructions.len() {
                i + 1
            } else {
                0
            };

            if (location, instruction) == step_cycle {
                return StepCycleData {
                    step_cycle,
                    num_steps,
                    term_nodes,
                };
            }
        }
    }
}

fn num_ghost_steps(document: &Document) -> usize {
    let mut locations: Vec<&str> = document.nodes.keys()
        .filter(|&k| k.ends_with("A"))
        .map(|&k| k)
        .collect();

    let step_cycle_datas = locations.iter()
        .map(|location| {
            let step_cycle = find_step_cycle(document, location);
            let step_cycle_data = get_step_cycle_data(document, step_cycle);
            step_cycle_data
        })
        .collect_vec();

    // StepCycleData { step_cycle: ("QHR", 2), num_steps: 19241, term_nodes: [("ZZZ", 270)] }
    // StepCycleData { step_cycle: ("HTX", 2), num_steps: 21409, term_nodes: [("QRZ", 270)] }
    // StepCycleData { step_cycle: ("SLJ", 4), num_steps: 11653, term_nodes: [("SLZ", 270)] }
    // StepCycleData { step_cycle: ("DQJ", 4), num_steps: 14363, term_nodes: [("FDZ", 270)] }
    // StepCycleData { step_cycle: ("GTT", 2), num_steps: 12737, term_nodes: [("XRZ", 270)] }
    // StepCycleData { step_cycle: ("TVD", 3), num_steps: 15989, term_nodes: [("HCZ", 270)] }
    step_cycle_datas.iter().for_each(|data| { debug!("{:?}", data) });

    let num_steps = step_cycle_datas.iter().map(|c| c.num_steps as usize).collect_vec();
    let num_steps_lcm = lcm(&num_steps[..]);

    debug!("step_cycle_instruction {:?}", num_steps_lcm);

    return num_steps_lcm;

    // too slow
    //
    // loop {
    //     for dir in document.instructions.chars().cycle() {
    //         for i in 0..locations.len() {
    //             let location = locations.get(i).unwrap();
    //             let node = document.nodes.get(location).unwrap();
    //             let new_location = match dir {
    //                 'L' => node.0,
    //                 'R' => node.1,
    //                 _ => panic!("Unknown character"),
    //             };
    //             let _ = mem::replace(&mut locations[i], new_location);
    //         }

    //         steps += 1;

    //         if locations.iter().all(|l| l.ends_with("Z")) {
    //             return steps;
    //         }

    //         debug!("{:?}", locations);
    //     }
    // }
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn part_two_test() {
    let input = InputSnake::new("test_input_3");
    let no_snake = input.no_snake();
    let (_, document) = parse_document(&no_snake).unwrap();

    let num_steps = num_ghost_steps(&document);
    info!("{:?}", num_steps);
}

fn part_two() {
    let input = InputSnake::new("input");
    let no_snake = input.no_snake();
    let (_, document) = parse_document(&no_snake).unwrap();

    let num_steps = num_ghost_steps(&document);
    info!("{:?}", num_steps);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"));

    info!("Part One Test");
    part_one_test();
    info!("Part One Test Two");
    part_one_test_two();
    info!("Part One");
    part_one();
 
    info!("Part Two Test");
    part_two_test();
    info!("Part Two");
    part_two();
}
