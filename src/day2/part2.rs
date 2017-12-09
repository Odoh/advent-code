use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

/// Returns the sum of all differences
pub fn checksum(file: &File) -> u32 {
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let numbers: Vec<u32> = line.unwrap()
                                    .split_whitespace()
                                    .map(|s| s.parse::<u32>())
                                    .filter_map(Result::ok)
                                    .collect();
        sum += divide(&numbers);
    }
    sum
}

/// Returns the division of the two divisible numbers in each row
fn divide(slice: &[u32]) -> u32 {
    if slice.len() < 1 {
        return 0;
    }
    let (dividend, divisor) = slice.iter()
                                    // find divisible value not equal to itself
                                   .filter_map(|v1| slice.iter()
                                                         .find(|&v2| (v2 != v1) && (v1 % v2 == 0))
                                                         .map(|v2| (v1, v2)))
                                   .next()
                                   .unwrap();
    dividend / divisor
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use super::checksum;

    #[test]
    fn examples() {
        let path = "src/day2/part2_example";
        let file = File::open(path).expect(&format!("{} not found", path));
        assert_eq!(checksum(&file), 9);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("part2 <filename>");
        return;
    }
    let filename: &str = &args[1];
    let file = File::open(filename).expect(&format!("{} not found", filename));
    println!("{}", checksum(&file));
}
