use itertools::Itertools;
use regex::Regex;

use std::num::ParseIntError;
use std::str::FromStr;

lazy_static! {
    // #1 @ 1,3: 4x4
    static ref CLAIM_REGEX: Regex = Regex::new(r"(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)").expect("Legal parse regex");
}


struct Fabric {
    size: usize,
    claim_ids: Vec<Vec<Vec<usize>>>,
}

struct Size {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

struct Claim {
    id: usize,
    size: Size,
}

impl Fabric {
    fn new(size: usize) -> Fabric {
        Fabric {
            size,
            claim_ids: vec![vec![Vec::new(); size]; size],
        }
    }

    fn add_claim(&mut self, claim: &Claim) {
        for (x, y) in claim.size.iter() {
            self.claim_ids[x][y].push(claim.id);
        }
    }

    fn overlap_area(&self) -> usize {
        self.iter()
            .fold(0, |acc, (x, y)| {
                if self.claim_ids[x][y].len() > 1 {
                    acc + 1
                } else {
                    acc
                }
            })
    }

    fn does_overlap(&self, claim_id: usize) -> bool {
        for (x, y) in self.iter() {
            if self.claim_ids[x][y].contains(&claim_id) &&
               self.claim_ids[x][y].len() > 1 {
                return true;
            }
        }
        return false;
    }

    fn iter(&self) -> impl Iterator<Item=(usize, usize)> {
        (0..self.size).cartesian_product(0..self.size)
    }
}

impl Size {
    fn iter(&self) -> impl Iterator<Item=(usize, usize)> {
        let xs = self.left..(self.left + self.width);
        let ys = self.top..(self.top + self.height);
        xs.cartesian_product(ys)
    }
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cs = CLAIM_REGEX.captures(s).expect("Regex capture match");
        let id = cs.name("id").unwrap().as_str().parse::<usize>()?;
        let left = cs.name("left").unwrap().as_str().parse::<usize>()?;
        let top = cs.name("top").unwrap().as_str().parse::<usize>()?;
        let width = cs.name("width").unwrap().as_str().parse::<usize>()?;
        let height = cs.name("height").unwrap().as_str().parse::<usize>()?;
        Ok(Claim {
            id,
            size: Size {
                left,
                width,
                top,
                height,
            }
        })
    }
}

pub fn main() {
    let f = include_str!("input_part1");

    let mut fabric = Fabric::new(1000);
    let claims = f.lines()
                  .map(|l| Claim::from_str(l).expect("Valid claim"))
                  .collect::<Vec<Claim>>();
    println!("number of claims {}", claims.len());

    claims.iter().for_each(|c| fabric.add_claim(&c));
    println!("Overlap area = {}", fabric.overlap_area());

    for (i, claim) in claims.iter().enumerate() {
        print!("claim {}/{} ... ", i + 1, claims.len());
        if fabric.does_overlap(claim.id) {
            println!(" overlaps");
            continue;
        }

        println!(" does NOT overlap -- id = {}", claim.id);
    }
}
