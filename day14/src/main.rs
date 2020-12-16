use log::{debug, info, warn, error};
use itertools::Itertools;
use env_logger;
use regex::Regex;
use byteorder::{LittleEndian, BigEndian, ByteOrder};
use bit_vec::BitVec;


use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::iter;

use advent::{InputSnake, FromRegex};

const MASK_REGEX: &str = r"^mask = ([X01]+)$";
const MEM_REGEX: &str = r"^mem\[(\d+)] = (\d+)$";

#[derive(Debug)]
struct BitMask {
    or_mask: u64,
    and_mask: u64,
}

impl BitMask {
    /// Parse the line "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
    pub fn from(line: &str) -> Self {
        let or_bitvec = iter::repeat('X')
            .take(28)
            .chain(line.chars())
            .map(|c| match c {
                'X'|'0' => false,
                '1' => true,
                _ => panic!("Unhandled character: {}", c),
            })
            .collect::<BitVec>();
        let and_bitvec = iter::repeat('X')
            .take(28)
            .chain(line.chars())
            .map(|c| match c {
                '0' => false,
                'X'|'1' => true,
                _ => panic!("Unhandled character: {}", c),
            })
            .collect::<BitVec>();

        debug!("{:?}  {:?}", or_bitvec, and_bitvec);
        let or_buf = or_bitvec.to_bytes();
        let and_buf = and_bitvec.to_bytes();
        debug!("{:?}  {:?}", or_buf, and_buf);

        BitMask {
            or_mask: BigEndian::read_u64(&or_buf),
            and_mask: BigEndian::read_u64(&and_buf),
        }
    }

    /// Return the value after the bit mask is applied
    pub fn mask(&self, value: u64) -> u64 {
        (value | self.or_mask) & self.and_mask
    }
}

#[derive(Debug)]
struct Memory {
    memory: HashMap<u64, u64>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: HashMap::new(),
        }
    }

    pub fn write(&mut self, address: u64, value: u64) {
        self.memory.insert(address, value);
    }

    pub fn sum_values(&self) -> u64 {
        self.memory.values().sum()
    }
}


fn part_one() {
    let mask_re = Regex::new(r"^mask = ([X01]+)$").unwrap();
    let mem_re = Regex::new(r"^mem\[(\d+)] = (\d+)$").unwrap();
    let mut memory = Memory::new();
    let mut bit_mask = None;
    for line in InputSnake::new("input").snake() {
        if mask_re.is_match(&line) {
            let mask_captures = mask_re.captures(&line).unwrap();
            let mask_str = mask_captures.get(1).unwrap().as_str();
            bit_mask = Some(BitMask::from(&mask_str));
            debug!("bit_mask: {:?}", bit_mask.as_ref().unwrap());
            continue;
        }

        let mem_captures = mem_re.captures(&line).unwrap();
        let address = mem_captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let value = mem_captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
        let masked_value = bit_mask.as_ref().unwrap().mask(value);
        debug!("address {} {} -> {}", address, value, masked_value);
        memory.write(address, masked_value);
    }

    info!("Part One: {:?}", memory.sum_values());
}

fn part_two() {
    let input = InputSnake::new("input");
    info!("Part Two: {:?}", 2);
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    part_one();
    part_two();
}
