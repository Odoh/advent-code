use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

pub fn main() {
    let mut freq = 0;
    let mut seen_freqs = hashset!{ freq };
    loop {
        let f = File::open("src/day1/input_part2").expect("Unable to open file");
        let f = BufReader::new(f);

        for line in f.lines() {
            let line = line.expect("Unable to read line");
            let change: i32 = line.parse().expect("Unable to parse line");

            freq += change;
            if seen_freqs.contains(&freq) {
                println!("Repeat frequency = {}", freq);
                return;
            }
            seen_freqs.insert(freq);
        }
    }
}
