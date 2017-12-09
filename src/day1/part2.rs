use std::env;

/// Returns the sum of all digits that match the next
/// digit in the slice; the slice is circular.
pub fn captcha(slice: &[u32]) -> u32 {
    if slice.len() == 0 {
        return 0;
    }

    let shifted = shift(slice, slice.len() / 2);
    slice.iter()
        .zip(shifted.iter())
        .filter(|&(v1, v2)| v1 == v2)
        .map(|(v1, _)| v1)
        .sum()
}

/// Construct a vector with all elements of a slice shifted
/// to the right by amount; the shifting is circular.
fn shift(slice: &[u32], amount: usize) -> Vec<u32> {
    let mut shifted = Vec::with_capacity(slice.len());
    shifted.extend_from_slice(&slice[amount..]);
    shifted.extend_from_slice(&slice[..amount]);
    shifted
}

#[cfg(test)]
mod test {
    use super::captcha;

    #[test]
    fn examples() {
        assert_eq!(captcha(&[1, 2, 1, 2]), 6);
        assert_eq!(captcha(&[1, 2, 2, 1]), 0);
        assert_eq!(captcha(&[1, 2, 3, 4, 2, 5]), 4);
        assert_eq!(captcha(&[1, 2, 3, 1, 2, 3]), 12);
        assert_eq!(captcha(&[1, 2, 1, 3, 1, 4, 1, 5]), 4);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("part2 <digits>");
        return;
    }
    let digits: Vec<u32> = args[1].chars().map(|c| c.to_digit(10).unwrap()).collect();
    println!("{}", captcha(&digits[..]));
}

