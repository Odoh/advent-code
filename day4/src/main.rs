use advent::InputSnake;
use itertools::Itertools;

mod password {
    use itertools::Itertools;
    use std::collections::HashSet;

    const VALIDATORS_PART1: [fn(&str) -> bool; 3] = [
        is_six_digits,
        contains_double_adjacent_digits,
        contains_never_decreasing_digits,
    ];

    const VALIDATORS_PART2: [fn(&str) -> bool; 3] = [
        is_six_digits,
        contains_double_adjacent_digits_not_in_larger_group,
        contains_never_decreasing_digits,
    ];

    pub fn is_valid_part1(pwd: &str) -> bool {
        VALIDATORS_PART1.iter().all(|v| v(pwd))
    }

    pub fn is_valid_part2(pwd: &str) -> bool {
        VALIDATORS_PART2.iter().all(|v| v(pwd))
    }

    fn is_six_digits(pwd: &str) -> bool {
        pwd.chars().count() == 6
    }

    fn contains_double_adjacent_digits(pwd: &str) -> bool {
        pwd.chars()
            .tuple_windows()
            .any(|(a, b)| a == b)
    }

    fn contains_never_decreasing_digits(pwd: &str) -> bool {
        pwd.chars()
            .tuple_windows()
            .all(|(a, b)| b >= a)
    }

    fn contains_double_adjacent_digits_not_in_larger_group(pwd: &str) -> bool {
        let mut double_digits_indices: HashSet<usize> = pwd.chars()
            .tuple_windows()
            .enumerate()
            .filter(|(_, (a, b))| a == b)
            .map(|(i, _)| i)
            .collect();
        let triple_digits_indices: HashSet<usize> = pwd.chars()
            .tuple_windows::<(_, _, _)>()
            .enumerate()
            .filter(|(_, (a, b, c))| a == b && b == c)
            .map(|(i, _)| i)
            .collect();

        triple_digits_indices.iter().for_each(|v| {
            double_digits_indices.remove(v);
            double_digits_indices.remove(&(v + 1));
        });
        !double_digits_indices.is_empty()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_contains_double_adjacent_digits() {
            assert!(contains_double_adjacent_digits("1223"));
            assert!(!contains_double_adjacent_digits("123"));
        }

        #[test]
        fn test_contains_never_decreasing_digits() {
            assert!(contains_never_decreasing_digits("1223"));
            assert!(!contains_never_decreasing_digits("121"));
        }

        #[test]
        fn test_contains_double_adjacent_digits_not_in_larger_group() {
            assert!(contains_double_adjacent_digits_not_in_larger_group("1223"));
            assert!(!contains_double_adjacent_digits_not_in_larger_group("12223"));
            assert!(contains_double_adjacent_digits_not_in_larger_group("122233"));

        }
    }
}

fn parse_password_range(s: &str) -> (i32, i32) {
    s.split('-')
        .map(|s| s.parse::<i32>().expect("Parse as i32"))
        .next_tuple()
        .unwrap()
}

fn part_one() {
    let input = InputSnake::new("input");
    let (start, end) = parse_password_range(&input.no_snake());
    let valid_passwords = (start..=end)
        .map(|p| p.to_string())
        .filter(|pwd| password::is_valid_part1(pwd))
        .count();
    println!("Part One: {:?}", valid_passwords);
}

fn part_two() {
    let input = InputSnake::new("input");
    let (start, end) = parse_password_range(&input.no_snake());
    let valid_passwords = (start..=end)
        .map(|p| p.to_string())
        .filter(|pwd| password::is_valid_part2(pwd))
        .count();
    println!("Part Two: {:?}", valid_passwords);
}

fn main() {
    part_one();
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_password_range() {
        assert_eq!((12, 23), parse_password_range("12-23"));
    }

    #[test]
    fn test_part_1() {
    }

    #[test]
    fn test_part_2() {
    }
}
