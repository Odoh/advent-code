#[derive(Debug)]
struct SpinLock {
    cur: usize,
    steps: usize,
    buf: Vec<usize>,
}

impl SpinLock {
    fn new(steps: usize) -> Self {
        SpinLock {
            cur: 0,
            steps,
            buf: vec![0],
        }
    }

    fn spin(&mut self) {
        // spin to find the next current position
        let buf_len = self.buf.len();
        let spin_pos = (self.cur + self.steps) % buf_len;
        self.cur = spin_pos + 1;

        // increase the spin buffer
        self.buf.insert(self.cur, buf_len);
    }
}

fn main() {
    // let mut example_spin_lock = SpinLock::new(3);
    // println!("{:?}", example_spin_lock);
    // example_spin_lock.spin();
    // println!("{:?}", example_spin_lock);
    // example_spin_lock.spin();
    // println!("{:?}", example_spin_lock);
    let mut spin_lock = SpinLock::new(369);
    for _ in 0..2017 {
        spin_lock.spin();
    }
    let pos = spin_lock.buf.iter().position(|&v| v == 2017).unwrap();
    println!("{}", spin_lock.buf[pos + 1]);
    // println!("{:?}", spin_lock);
}
