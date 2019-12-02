use regex::Regex;

use std::collections::HashMap;
use std::ops::Range;

lazy_static! {
    // [1518-11-01 00:00] Guard #10 begins shift
    static ref SHIFT_REGEX: Regex = Regex::new(r"Guard #(?P<id>\d+) begins shift").expect("Shift Regex format");

    // [1518-11-01 00:05] falls asleep
    static ref SLEEP_REGEX: Regex = Regex::new(r":(?P<min>\d+)\] falls asleep").expect("Sleep Regex format");

    // [1518-11-01 00:25] wakes up
    static ref WAKE_REGEX: Regex = Regex::new(r":(?P<min>\d+)\] wakes up").expect("Wake Regex format");
}

const AWAKE: bool = true;
const ASLEEP: bool = false;

struct Guard {
    id: usize,
    shift_minutes: Vec<[bool; 60]>,
}

impl Guard {
    pub fn new(id: usize) -> Self {
        Guard {
            id,
            shift_minutes: Vec::new(),
        }
    }

    pub fn start_shift(&mut self) {
        self.shift_minutes.push([AWAKE; 60]);
    }

    pub fn awake_minutes(&mut self, minutes: Range<usize>) {
        self.status_minutes(minutes, AWAKE);
    }

    pub fn sleep_minutes(&mut self, minutes: Range<usize>) {
        self.status_minutes(minutes, ASLEEP);
    }

    fn status_minutes(&mut self, minutes: Range<usize>, status: bool) {
        let day = self.shift_minutes.last().unwrap();
        for minute in minutes {
            day[minute] = status;
        }
    }
}

struct Parser {
    // store all guards parsed
    parsed_guards: HashMap<usize, Guard>,

    // store the guard currently being parsed
    last_guard: Option<Guard>,
    last_status: bool,
    last_minute: usize,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            parsed_guards: HashMap::new(),

            last_guard: None,
            last_status: AWAKE,
            last_minute: 0,
        }
    }

    pub fn parse_line(&mut self, s: &str) {
        if SHIFT_REGEX.is_match(s) {
            return;
        }
    }

    fn parse_shift(&mut self, s: &str) {
        // fill
        let c = SHIFT_REGEX.captures(s).expect("Shift Regex captures");
        let id = c.name("id").unwrap().as_str().parse::<usize>().expect("Parse ID");
        // self.last_guard = 
    }

    fn complete_shift(&mut self) {
        if let Some(ref guard) = self.last_guard {
            
        }
    }
}



// impl Guard {
// }

pub fn main() {
    let f = include_str!("input_part1");

    println!("Day 4 Part 1");
}
