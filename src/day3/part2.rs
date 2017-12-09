use std::env;

/// Defines a layout of SpiralMemory
pub mod spiral_memory {

    /*
     * Each square on the grid is allocated in a spiral pattern starting at a location marked 1
     * and then counting up while spiraling outward. For example:
     *     5   4   3
     *     6   1   2
     *     7   8   9 -->
     */

    /// The memory structure
    pub struct Mem {
        pub access_point: Loc,
    }

    /// A memory location
    #[derive(Clone, Copy)]
    pub struct Loc {
        pub square: u32,
        pub store: u32,
        x: i32,
        y: i32,

        // track where we are in the spiral to be able
        // to construct the next memory location
        dir: Dir,
    }

    /// A direction of travel for the spiral
    ///
    /// contains the max distance allowed to travel before
    /// requiring a change in direction
    #[derive(Clone, Copy)]
    enum Dir {
        IncX(u32),
        DecX(u32),
        IncY(u32),
        DecY(u32),
    }

    pub struct LocIter {
        iters: Vec<Loc>
    }

    impl Mem {
        pub fn new() -> Self {
            Mem {
                access_point: Loc {
                    square: 1,
                    store: 1,
                    x: 0,
                    y: 0,
                    dir: Dir::IncX(1),
                },
            }
        }

        pub fn iter(&self) -> LocIter {
            LocIter { iters: vec!(self.access_point) }
        }

        /// Return the distance of square from the access point.
        pub fn distance(&self, square: u32) -> u32 {
            if square == 1 {
                return 0;
            }
            let loc = self.iter().find(|loc| loc.square == square).unwrap();
            (loc.x.abs() + loc.y.abs()) as u32
        }

        /// Return the stored value of square.
        pub fn store(&self, square: u32) -> u32 {
            if square == 1 {
                return 1;
            }
            let loc = self.iter().find(|loc| loc.square == square).unwrap();
            loc.store
        }
    }

    impl LocIter {
        fn adjacent_locs(&self, x: i32, y: i32) -> Vec<Loc> {
            let mut locs: Vec<Loc> = vec!();
            for loc in self.iters.iter() {
                if (loc.x - x).abs() <= 1 &&
                   (loc.y - y).abs() <= 1 {
                       locs.push(loc.clone());
                   }
            }
            locs
        }
    }

    impl Iterator for LocIter {
        type Item = Loc;

        fn next(&mut self) -> Option<Self::Item> {
            let loc = self.iters.last().cloned().unwrap();
            let next_square = loc.square + 1;
            let (next_x, next_y, next_dir) = match loc.dir {
                Dir::IncX(limit) => {
                    let next_x = loc.x + 1;
                    let next_y = loc.y;
                    let next_dir = if (next_x.abs() as u32) == limit { loc.dir.next() }
                                   else { loc.dir };
                    (next_x, next_y, next_dir)
                },
                Dir::IncY(limit) => {
                    let next_x = loc.x;
                    let next_y = loc.y + 1;
                    let next_dir = if (next_y.abs() as u32) == limit { loc.dir.next() }
                                   else { loc.dir };
                    (next_x, next_y, next_dir)
                },
                Dir::DecX(limit) => {
                    let next_x = loc.x - 1;
                    let next_y = loc.y;
                    let next_dir = if (next_x.abs() as u32) == limit { loc.dir.next() }
                                   else { loc.dir };
                    (next_x, next_y, next_dir)
                },
                Dir::DecY(limit) => {
                    let next_x = loc.x;
                    let next_y = loc.y - 1;
                    let next_dir = if (next_y.abs() as u32) == limit { loc.dir.next() }
                                   else { loc.dir };
                    (next_x, next_y, next_dir)
                },
            };
            let next_loc = Loc {
                square: next_square,
                store: self.adjacent_locs(next_x, next_y).iter().map(|l| l.store).sum(),
                x: next_x,
                y: next_y,
                dir: next_dir,
            };
            self.iters.push(next_loc);
            Some(next_loc)
        }
    }

    impl Dir {
        fn next(self) -> Dir {
            // sprial is built in order: +x, +y, -x, -y
            // the limit of travel distance does change until the order resets
            match self {
                Dir::IncX(limit) => Dir::IncY(limit),
                Dir::IncY(limit) => Dir::DecX(limit),
                Dir::DecX(limit) => Dir::DecY(limit),
                Dir::DecY(limit) => Dir::IncX(limit + 1),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::spiral_memory;

    #[test]
    fn examples() {
        let mem = spiral_memory::Mem::new();
        assert_eq!(mem.store(1), 1);
        assert_eq!(mem.store(2), 1);
        assert_eq!(mem.store(3), 2);
        assert_eq!(mem.store(4), 4);
        assert_eq!(mem.store(5), 5);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("part1 <store: u32>");
        return;
    }
    let store = args[1].parse::<u32>().expect("part1 <store: u32>");
    let mem = spiral_memory::Mem::new();
    let square = (1..).find(|&s| mem.store(s) > store).unwrap();
    println!("{}", mem.store(square));
}
