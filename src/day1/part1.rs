use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let f = File::open("src/day1/input_part1").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut freq = 0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let change: i32 = line.parse().expect("Unable to parse line");
        freq += change
    }

    println!("Frequency = {}", freq);
}
