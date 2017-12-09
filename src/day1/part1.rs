use std::env;

/// Returns the sum of all digits that match the next
/// digit in the slice; the slice is circular.
pub fn captcha(slice: &[u32]) -> u32 {
    if slice.len() == 0 {
        return 0;
    }

    // construct slice with all values shifted by 1 
    let mut shifted = Vec::with_capacity(slice.len());
    shifted.push(slice[slice.len() - 1]);
    shifted.extend_from_slice(slice);
    shifted.pop();

    slice.iter()
        .zip(shifted.iter())
        .filter(|&(v1, v2)| v1 == v2)
        .map(|(v1, _)| v1)
        .sum()
}

#[cfg(test)]
mod test {
    use super::captcha;

    #[test]
    fn examples() {
        assert_eq!(captcha(&[1, 1, 2, 2]), 3);
        assert_eq!(captcha(&[1, 1, 1, 1]), 4);
        assert_eq!(captcha(&[1, 2, 3, 4]), 0);
        assert_eq!(captcha(&[9, 1, 2, 1, 2, 1, 2, 9]), 9);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("part1 <digits>");
        return;
    }
    let digits: Vec<u32> = args[1].chars().map(|c| c.to_digit(10).unwrap()).collect();
    println!("{}", captcha(&digits[..]));
}

