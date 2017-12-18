#[derive(Debug)]
struct AngrySpinLock {
    cur: usize,
    steps: usize,
    buf_len: usize,
    after_0: Option<usize>,
}

impl AngrySpinLock {
    fn new(steps: usize) -> Self {
        AngrySpinLock {
            cur: 0,
            steps,
            buf_len: 1,
            after_0: None,
        }
    }

    fn spin(&mut self) {
        // spin to find the next current position
        let spin_pos = (self.cur + self.steps) % self.buf_len;
        self.cur = spin_pos + 1;

        // value after 0 updated
        if self.cur == 1 {
            self.after_0 = Some(self.buf_len);
        }

        self.buf_len += 1;
    }
}

fn main() {
    let mut spin_lock = AngrySpinLock::new(369);
    for _ in 0..50_000_000 {
        spin_lock.spin();
    }
    println!("{}", spin_lock.after_0.unwrap());
}
