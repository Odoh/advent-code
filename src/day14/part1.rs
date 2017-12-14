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

fn main() {
    // let key = "flqrgnkx";
    let key = "hfdlxzhv";
    let squares = frag(key);
    println!("{}", squares);
}
