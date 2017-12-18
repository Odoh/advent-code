
/*
 *
 * There are sixteen programs in total, named a through p. They start by standing in a line: a stands in position 0, b stands in position 1, and so on until p, which stands in position 15.
 * 
 * The programs' dance consists of a sequence of dance moves:
 * 
 * Spin, written sX, makes X programs move from the end to the front, but maintain their order otherwise. (For example, s3 on abcde produces cdeab).
 * Exchange, written xA/B, makes the programs at positions A and B swap places.
 * Partner, written pA/B, makes the programs named A and B swap places.
 * 
 * For example, with only five programs standing in a line (abcde), they could do the following dance:
 * 
 * s1, a spin of size 1: eabcd.
 * x3/4, swapping the last two programs: eabdc.
 * pe/b, swapping programs e and b: baedc.
 * 
 * After finishing their dance, the programs end up in order baedc.
 * 
 * You watch the dance for a while and record their dance moves (your puzzle input). In what order are the programs standing after their dance?
 */

struct Programs(Vec<char>);

impl Programs {
    fn new(input: &str) -> Self {
        Programs(input.chars().collect())
    }

    fn to_string(&self) -> String {
        self.0.iter().collect()
    }

    fn spin(&mut self, size: usize) {
        self.0 = {
            let mut new_vec = Vec::with_capacity(self.0.len());
            let (begin, end) = self.0.split_at(self.0.len() - size);
            new_vec.extend_from_slice(end);
            new_vec.extend_from_slice(begin);
            new_vec
        };
    }

    fn swap_indices(&mut self, i1: usize, i2: usize) {
        self.0.swap(i1, i2);
    }

    fn swap_programs(&mut self, p1: char, p2: char) {
        let i1 = self.0.iter()
                       .position(|&p| p == p1)
                       .unwrap();
        let i2 = self.0.iter()
                       .position(|&p| p == p2)
                       .unwrap();
        self.swap_indices(i1, i2);
    }
}

fn dance(programs: &mut Programs, moves: &str) {
    for mv in moves.split(',') {
        let mv_type = mv.chars().next().unwrap();
        match mv_type {
            's' => {
                let size = &mv[1..].parse::<usize>().unwrap();
                programs.spin(*size);
            },
            'x' => {
                let indices: Vec<usize> = (&mv[1..]).split('/')
                                                    .map(|s| s.parse::<usize>().unwrap())
                                                    .collect();
                programs.swap_indices(indices[0], indices[1]);
            },
            'p' => {
                let names: Vec<char> = (&mv[1..]).split('/')
                                                 .map(|s| s.parse::<char>().unwrap())
                                                 .collect();
                programs.swap_programs(names[0], names[1]);
            },
             _ => panic!("Unsupported move char {}", mv_type),
        }
        
    }
}

fn main() {
    // let example = "abcde";
    let question = "abcdefghijklmnop";
    let mut programs = Programs::new(question);
    // let moves = "s1,x3/4,pe/b";
    let moves = include_str!("question").trim();
}
