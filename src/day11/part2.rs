use std::cmp;
use std::ops::Add;

#[derive(Debug)]
struct Path {
    nw: u32,
    n: u32,
    ne: u32,
    sw: u32,
    s: u32,
    se: u32,
}

enum Dir {
    NW,
    N,
    NE,
    SW,
    S,
    SE,
}

impl Add<Dir> for Path {
    type Output = Path;

    fn add(self, dir: Dir) -> Path {
        match dir {
            Dir::NW => {
                match self {
                    _ if self.s > 0 => Path { s: self.s - 1, .. self } + Dir::SW,
                    _ if self.se > 0 => Path { se: self.se - 1, .. self },
                    _ if self.ne > 0 => Path { ne: self.ne - 1, .. self } + Dir::N,
                    _ => Path { nw: self.nw + 1, .. self },
                }
            },
            Dir::N => {
                match self {
                    _ if self.sw > 0 => Path { sw: self.sw - 1, .. self } + Dir::NW,
                    _ if self.s > 0 => Path { s: self.s - 1, .. self },
                    _ if self.se > 0 => Path { se: self.se - 1, .. self } + Dir::NE,
                    _ => Path { n: self.n + 1, .. self },
                }
            },
            Dir::NE => {
                match self {
                    _ if self.nw > 0 => Path { nw: self.nw - 1, .. self } + Dir::N,
                    _ if self.sw > 0 => Path { sw: self.sw - 1, .. self },
                    _ if self.s > 0 => Path { s: self.s - 1, .. self } + Dir::SE,
                    _ => Path { ne: self.ne + 1, .. self },
                }
            },
            Dir::SW => {
                match self {
                    _ if self.n > 0 => Path { n: self.n - 1, .. self } + Dir::NW,
                    _ if self.ne > 0 => Path { ne: self.ne - 1, .. self },
                    _ if self.se > 0 => Path { se: self.se - 1, .. self } + Dir::S,
                    _ => Path { sw: self.sw + 1, .. self },
                }
            },
            Dir::S => {
                match self {
                    _ if self.nw > 0 => Path { nw: self.nw - 1, .. self } + Dir::SW,
                    _ if self.n > 0 => Path { n: self.n - 1, .. self },
                    _ if self.ne > 0 => Path { ne: self.ne - 1, .. self } + Dir::SE,
                    _ => Path { s: self.s + 1, .. self },
                }
            },
            Dir::SE => {
                match self {
                    _ if self.sw > 0 => Path { sw: self.sw - 1, .. self } + Dir::S,
                    _ if self.nw > 0 => Path { nw: self.nw - 1, .. self },
                    _ if self.n > 0 => Path { n: self.n - 1, .. self } + Dir::NE,
                    _ => Path { se: self.se + 1, .. self },
                }
            },
        }
    }
}

fn steps(input: &str) -> (u32, u32) {
    let start = Path { nw: 0, n: 0, ne: 0,
                       sw: 0, s: 0, se: 0, };
    let (dest, max) = input.split(',')
                           .fold((start, 0), |(path, max), dir| {
                               let d = path + match dir {
                                     "nw" => Dir::NW,
                                     "n" => Dir::N,
                                     "ne" => Dir::NE,
                                     "se" => Dir::SE,
                                     "s" => Dir::S,
                                     "sw" => Dir::SW,
                                     _ => panic!("Unhandled direction: {:?}", dir)
                                 };
                               let m = cmp::max(max, d.nw + d.n + d.ne + d.sw + d.s + d.se);
                               (d, m)
                           });
    println!("{:?} {}", dest, max);
    (dest.nw + dest.n + dest.ne + dest.sw + dest.s + dest.se, max)
}

fn main() {
    assert_eq!(steps("ne,ne,ne"), (3, 3));
    assert_eq!(steps("ne,ne,sw,sw"), (0, 2));
    assert_eq!(steps("ne,ne,s,s"), (2, 2));
    assert_eq!(steps("se,sw,se,sw,sw"), (3, 3));

    let question = include_str!("question").trim();
    println!("{:?}", steps(question));
}
