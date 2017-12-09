use std::collections::HashSet;

#[derive(Clone, Hash)]
struct MemBank(Vec<u32>);

impl MemBank {
    fn new(blocks: Vec<u32>) -> MemBank {
        MemBank(blocks)
    }

    fn iter(&self) -> MemBankIterator {
        MemBankIterator {
            blocks: self.0.clone(),
            prev_blocks: HashSet::new(),
            infinite_loop: false,
        }
    }
}

impl Iterator for MemBankIterator {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        // if an infinite loop has been detected, stop
        if self.infinite_loop {
            return None;
        }

        // find the max bank with the min index
        let (i, v): (usize, u32) = self.blocks.iter()
                                              .enumerate()
                                              .max_by(|&(i1, v1), &(i2, v2)|
                                                      v1.cmp(v2).then(i2.cmp(&i1)))
                                              .map(|(i, v)| (i, v.clone())) // stop borrowing
                                              .unwrap();

        // redistribute the wealth
        let mut next_blocks = self.blocks.clone();
        let blocks_len = next_blocks.len();
        next_blocks[i] = 0;
        for j in (i + 1)..(i + 1 + (v as usize)) {
            next_blocks[j % blocks_len] += 1;
        }

        // update iter context
        self.blocks = next_blocks.clone();
        self.infinite_loop = self.prev_blocks.contains(&next_blocks);
        self.prev_blocks.insert(next_blocks.clone());

        Some(next_blocks)
    }
}

struct MemBankIterator {
    blocks: Vec<u32>,
    prev_blocks: HashSet<Vec<u32>>,
    infinite_loop: bool,
}

#[cfg(test)]
mod test {

    use super::MemBank;

    #[test]
    fn example() {
        let blocks = vec![0, 2, 7, 0];
        let mem_bank = MemBank::new(blocks);
        let mut iter = mem_bank.iter();
        assert_eq!(iter.next(), Some(vec![2, 4, 1, 2]));
        assert_eq!(iter.next(), Some(vec![3, 1, 2, 3]));
        assert_eq!(iter.next(), Some(vec![0, 2, 3, 4]));
        assert_eq!(iter.next(), Some(vec![1, 3, 4, 1]));
        assert_eq!(iter.next(), Some(vec![2, 4, 1, 2]));
        assert_eq!(iter.next(), None);
        
        assert_eq!(mem_bank.iter().count(), 5);
    }
}

fn main() {
    let blocks: Vec<u32> = include_str!("question").split_whitespace()
                                                   .map(|s| s.parse::<u32>())
                                                   .filter_map(Result::ok)
                                                   .collect();
    let mem_bank = MemBank::new(blocks);
    println!("{}", mem_bank.iter().count());
}
