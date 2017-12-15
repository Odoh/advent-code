
struct Gen {
    init_value: u64,
    factor: u64,
    multiple: u64,
}

struct GenIter<'a> {
    value: u64,
    gen: &'a Gen,
}

impl Gen {
    fn new(init_value: u64, factor: u64, multiple: u64) -> Gen {
        Gen {
            init_value,
            factor,
            multiple,
        }
    }

    fn iter(&self) -> GenIter {
        GenIter {
            value: self.init_value,
            gen: &self,
        }
    }
}

impl<'a> Iterator for GenIter<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_value = self.value * self.gen.factor % 2147483647;
            self.value = next_value;
            if next_value % self.gen.multiple == 0 {
                return Some(next_value)
            }
        }
    }
}

fn matches(gen_a: &Gen, gen_b: &Gen) -> usize {
    gen_a.iter()
         .zip(gen_b.iter())
         .take(5_000_000)
         .filter(|&(a, b)| (a & 0xffff) == (b & 0xffff))
         .count()
}

fn main() {
    // let gen_a = Gen::new(65, 16807, 4);
    // let gen_b = Gen::new(8921, 48271, 8);
    let gen_a = Gen::new(516, 16807, 4);
    let gen_b = Gen::new(190, 48271, 8);
    let matches = matches(&gen_a, &gen_b);
    println!("{}", matches);
}
