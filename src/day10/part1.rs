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

    fn hash(&self) -> u32 {
        self.list[0] * self.list[1]
    }
}

fn main() {
    let question = vec!(120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113);

    let mut knot_hash = KnotHash::new();
    for length in question.iter() {
        knot_hash.tie(*length);
    }
    println!("{}", knot_hash.hash());
}
