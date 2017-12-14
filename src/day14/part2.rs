
use std::cmp;
use std::fmt;

mod knothash;

fn frag(key: &str) -> u32 {
    let mut total_ones = 0;
    for row in 0..128 {
        let row_key = format!("{}-{}", key, row);
        let hash_vec = knothash::knot_hash(&row_key);
        let ones = hash_vec.iter()
                           .map(|b| b.count_ones())
                           .sum();
        total_ones += ones;
    }
    total_ones
}

type Memory = [[bool; 128]; 128];

fn print(memory: &Memory) {
    for r in 0..128 {
        for c in 0..128 {
            print!("{} ", if memory[r][c] { "#" } else { "." });
        }
        println!();
    }
}

fn from_key(key: &str) -> Memory {
    let mut memory = [[false; 128]; 128];
    for row in 0..128 {
        let row_key = format!("{}-{}", key, row);
        let hash_u8s = knothash::knot_hash(&row_key);
        let hash_u1s = hash_u8s.iter()
                                // to u4
                               .flat_map(|b| vec!((b & 0xf0) >> 4,
                                                  (b & 0x0f) >> 0,))
                               // to u1
                               .flat_map(|b| vec!((b & 0b1000) >> 3,
                                                  (b & 0b0100) >> 2,
                                                  (b & 0b0010) >> 1,
                                                  (b & 0b0001) >> 0,))
                               .collect::<Vec<u32>>();
        for (c, &bit) in hash_u1s.iter().enumerate() {
            memory[row][c] = bit == 1
        }
    }
    memory
}

/// Kill oneself and all neighbors, recursively.
/// Return whether we killed anyone (including ourselves)
fn killing_spree(memory: &mut Memory, r: i32, c: i32) -> bool {
    // we're already dead
    if !memory[r as usize][c as usize] {
        return false;
    }

    // kill ourselves
    memory[r as usize][c as usize] = false;

    // and all our friends
    killing_spree(memory, r, cmp::max(0, c - 1)); // left
    killing_spree(memory, r, cmp::min(127, c + 1)); // right
    killing_spree(memory, cmp::max(0, r - 1), c); // up
    killing_spree(memory, cmp::min(127, r + 1), c); // down
    true
}

/// Return the number of regions in memory.
fn regions(memory: &mut Memory) -> u32 {
    let mut regions = 0;

    // commit genocide
    loop {
        for r in 0..128 {
            for c in 0..128 {
                if killing_spree(memory, r, c) {
                    regions += 1;
                    continue;
                }
            }
        }
        break;
    }
    regions
}

fn main() {
    // let key = "flqrgnkx";
    let key = "hfdlxzhv";
    let mut memory = from_key(key);
    // print(&memory);
    let regions = regions(&mut memory);
    println!("{}", regions);
}
