use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::CaptureMatches;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use advent::{InputSnake, FromRegex};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Operation {
    Accumulate,
    Jump,
    Noop,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    operation: Operation,
    argument: i64
}

impl Instruction {
    pub fn from(line: &str) -> Self {
        let mut split = line.split_whitespace();
        let operation = match split.next().unwrap() {
            "acc" => Operation::Accumulate,
            "jmp" => Operation::Jump,
            "nop" => Operation::Noop,
            op => panic!("Unhandled operation value: {}", op),
        };
        let argument = split.next().unwrap().parse::<i64>().unwrap();
        Instruction {
            operation,
            argument,
        }
    }
}

struct Console {
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    accumulator: i64,
    visited_instructions: Vec<bool>,
}

impl Console {
    const IGNORE_INSTRUCTION_POINTER_INCREMENT: [Operation; 1] = [Operation::Jump];

    pub fn new(instructions: Vec<Instruction>) -> Self {
        let instructions_len = instructions.len();
        Console {
            instructions,
            instruction_pointer: 0,
            accumulator: 0,
            visited_instructions: vec![false; instructions_len],
        }
    }

    pub fn run_until_loop(&mut self) -> Result<(), ()> {
        loop {
            let ip = self.instruction_pointer;
            if ip >= self.visited_instructions.len() {
                // the program ended
                return Ok(());
            }
            if self.visited_instructions[ip] {
                // the program hit an infinite loop
                return Err(());
            }
            let instruction = self.instructions[ip];
            self.execute(instruction);
            self.visited_instructions[ip] = true;
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        debug!("[{}] {:?} {:?}", self.instruction_pointer, instruction.operation, instruction.argument);
        match instruction.operation {
            Operation::Accumulate => self.accumulator += instruction.argument,
            Operation::Jump => self.instruction_pointer = ((self.instruction_pointer as i64) + instruction.argument) as usize,
            Operation::Noop => (),
        }
        if !Console::IGNORE_INSTRUCTION_POINTER_INCREMENT.contains(&instruction.operation) {
            self.instruction_pointer += 1;
        }
    }
}

fn part_one() {
    let instructions = InputSnake::new("input")
        .snake()
        .map(|l| Instruction::from(&l))
        .collect::<Vec<Instruction>>();

    instructions.iter().for_each(|i| debug!("{:?}", i));

    let mut console = Console::new(instructions);
    console.run_until_loop();

    info!("Part One: {:?}", console.accumulator);
}

fn part_two() {
    let base_instructions = InputSnake::new("input")
        .snake()
        .map(|l| Instruction::from(&l))
        .collect::<Vec<Instruction>>();

    for i in 1..base_instructions.len() {
        let mut instructions = base_instructions.clone();
        let instruction = instructions[i];
        match instruction.operation {
            Operation::Jump => instructions[i].operation = Operation::Noop,
            Operation::Noop => instructions[i].operation = Operation::Jump,
            _ => (),
        };

        let mut console = Console::new(instructions);
        let result = console.run_until_loop();
        if result.is_ok() {
            info!("Part Two: {:?}", console.accumulator);
            return;
        }
    }

}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"));
    part_one();
    part_two();
}
