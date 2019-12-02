use std::collections::HashMap;

fn similar_words(words: &str, num_diffs: usize) -> (&str, &str) {
    let mut similar_words = Vec::new();
    for word in words.lines() {
        for other in words.lines() {
            let mut diffs = 0;
            for (cw, co) in word.chars().zip(other.chars()) {
                if cw == co {
                    continue;
                }

                diffs += 1;

                if diffs > num_diffs {
                    break;
                }
            }
            if diffs <= num_diffs && diffs != 0 {
                similar_words.push(word);
                similar_words.push(other);
            }
        }
    }
    return (similar_words[0], similar_words[1]);
}

fn commonality(word1: &str, word2: &str) -> String {
    let mut word = Vec::new();
    for (c1, c2) in word1.chars().zip(word2.chars()) {
        if c1 == c2 {
            word.push(c2)
        }
    }
    return word.into_iter().collect();
}

pub fn main() {
    let f = include_str!("input_part2");

    let (word, other) = similar_words(f, 1);
    let commonality = commonality(word, other);

    println!("Commonality = {}", commonality);
}
