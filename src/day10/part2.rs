#[derive(Debug)]
struct KnotHash {
    list: Vec<u32>,
    cur: usize,
    skip: usize,
}

impl KnotHash {
    pub fn new() -> Self {
        KnotHash {
            list: (0..256).collect(),
            cur: 0,
            skip: 0,
        }
    }

    fn tie(&mut self, length: usize) {
        // reverse region of size length
        let mut region: Vec<u32> = self.list.iter()
                                            .cycle()
                                            .skip(self.cur)
                                            .take(length)
                                            .map(|v| *v)
                                            .collect();
        region.reverse();

        // construct iterator for updating self.list
        // (<index to update>, <value to update with>)
        let iter: Vec<(usize, u32)> = (0..length).map(|i| (i + self.cur) % self.list.len())
                                                 .zip(region.into_iter())
                                                 .collect();
        for (i, r) in iter {
            self.list[i] = r;
        }

        // move current position and increase skip size
        self.cur = (self.cur + length + self.skip) % self.list.len();
        self.skip += 1;
    }
}

fn knot_hash(input: &str) -> String {
    // convert the input string into its ascii codes
    let ascii: Vec<usize> = input.chars()
                                 .map(|c| c as usize)
                                 .collect();

    // add length suffix of '17, 31, 73, 47, 23'
    let mut lengths: Vec<usize> = ascii.clone();
    lengths.extend(vec!(17, 31, 73, 47, 23));

    // run 64 rounds of knot hashing
    let mut knot_hash = KnotHash::new();
    for _ in 0..64 {
        for l in lengths.iter() {
            knot_hash.tie(*l);
        }
    }

    // knot hash list now contains the sparse hash
    let sparse_hash = knot_hash.list.clone();

    // calcuate dense_hash by xor each section of 16 digits
    let dense_hash = sparse_hash.chunks(16)
                                .map(|cs| cs.iter()
                                            .skip(1)
                                            .fold(*cs.first().unwrap(), |acc, &x| acc ^ x))
                                .collect::<Vec<u32>>();
    // create hex string
    let hash: String = dense_hash.iter()
                                 .map(|block| format!("{:02x}", block))
                                 .collect();

    println!("ascii: {:?}", ascii);
    println!("lengths: {:?}", lengths);
    println!("sparse hash [{}]: {:?}", sparse_hash.len(), sparse_hash);
    println!("dense hash [{}]: {:?}", dense_hash.len(), dense_hash);
    hash
}

#[test]
fn examples() {
    assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}

fn main() {
    let input = "120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113";
    println!("{}", knot_hash(input));

}
