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
    // sort the characters of each word - handles anagrams
    let mut char_words: Vec<Vec<char>> = slice.iter()
                                              .map(|w| w.chars().collect::<Vec<char>>())
                                              .collect();
    for char_word in char_words.iter_mut() {
        char_word.sort();
    }

    // convert to a Set of Strings - handles duplicate words
    let str_words: HashSet<String> = char_words.iter()
                                               .map(|i| i.into_iter().collect())
                                               .collect();

    str_words.len() == slice.len()
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use super::valid_passphrase;

    #[test]
    fn examples() {
        assert_eq!(valid_passphrase(&"abcde fghij".split_whitespace().collect::<Vec<_>>()), true);
        assert_eq!(valid_passphrase(&"abcde xyz ecdab".split_whitespace().collect::<Vec<_>>()), false);
        assert_eq!(valid_passphrase(&"a ab abc abd abf abj".split_whitespace().collect::<Vec<_>>()), true);
        assert_eq!(valid_passphrase(&"iiii oiii ooii oooi oooo".split_whitespace().collect::<Vec<_>>()), true);
        assert_eq!(valid_passphrase(&"oiii ioii iioi iiio".split_whitespace().collect::<Vec<_>>()), false);
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
