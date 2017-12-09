use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::collections::HashSet;

/// Returns the number of valid passphrases.
pub fn num_valid_passphrases(file: &File) -> u32 {
    let reader = BufReader::new(file);
    let mut num = 0;
    for line in reader.lines() {
        // as_ref to prevent unwrap() from consuming line and then falling out of scope
        let words: Vec<&str> = line.as_ref()
                                   .unwrap()
                                   .split_whitespace()
                                   .collect();
        if valid_passphrase(&words) {
            num += 1;
        }
    }
    num
}

/// Returns whether slice is a valid passphrase.
fn valid_passphrase(slice: &[&str]) -> bool {
    let mut set = HashSet::new();
    for word in slice.iter() {
        set.insert(word);
    }
    set.len() == slice.len()
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use super::valid_passphrase;

    #[test]
    fn examples() {
        assert_eq!(valid_passphrase(&"aa bb cc dd ee".split_whitespace().collect::<Vec<_>>()), true);
        assert_eq!(valid_passphrase(&"aa bb cc dd aa".split_whitespace().collect::<Vec<_>>()), false);
        assert_eq!(valid_passphrase(&"aa bb cc dd aaa".split_whitespace().collect::<Vec<_>>()), true);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("part1 <filename>");
        return;
    }
    let filename: &str = &args[1];
    let file = File::open(filename).expect(&format!("{} not found", filename));
    println!("{}", num_valid_passphrases(&file));
}
