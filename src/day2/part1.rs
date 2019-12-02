use std::collections::HashMap;

fn char_count(s: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    map
}

fn twos_threes(s: &str) -> (u32, u32) {
    let counts = char_count(s);
    if counts.values().any(|&v| v == 2) && counts.values().any(|&v| v == 3) {
        return (1, 1);
    } else if counts.values().any(|&v| v == 2) {
        return (1, 0);
    } else if counts.values().any(|&v| v == 3) {
        return (0, 1);
    } else {
        return (0, 0);
    }
}

pub fn main() {
    let f = include_str!("input_part1");

    let mut total_twos = 0;
    let mut total_threes = 0;
    for line in f.lines() {
        let (twos, threes) = twos_threes(line);
        total_twos += twos;
        total_threes += threes;
    }

    println!("Checksum = {}", total_twos * total_threes);
}
