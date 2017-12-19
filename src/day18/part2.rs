use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::VecDeque;

struct SoundCard {
    pid: i64,
    registers: HashMap<char, i64>,
    instructions: Vec<Instruction>,
    cur: usize,
    snd_count: usize,
    snd_value: i64,
    rcv_queue: VecDeque<i64>,
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    /// Plays a sound with the freqency of X.
    Snd(char),
    /// Set register X to value.
    Set(char, i64),
    Setr(char, char),
    /// Add and set value to register X.
    Add(char, i64),
    Addr(char, char),
    /// Multiply and set value to register X.
    Mul(char, i64),
    Mulr(char, char),
    /// Mod and set value to register X.
    Mod(char, i64),
    Modr(char, char),
    /// Recovers the last played frequency, only if X > 0.
    Rcv(char),
    /// Jumps the offset of value, only if X > 0
    Jgz(char, i64),
    Jgzr(char, char),
}

impl SoundCard {
    fn new(instructions: Vec<Instruction>, pid: i64) -> Self {
        let mut registers = HashMap::new();
        registers.insert('p', pid);
        SoundCard {
            pid,
            registers,
            instructions,
            cur: 0,
            snd_count: 0,
            rcv_queue: VecDeque::new(),
            snd_value: 0,
        }
    }

    fn is_deadlocked(&self) -> bool {
        match self.instructions[self.cur] {
            Instruction::Rcv(_) => self.rcv_queue.is_empty(),
            _ => false
        }
    }
}

impl Iterator for SoundCard {
    // snd_count
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        // ends when there's an invalid current execution point
        if self.cur > self.instructions.len() {
            return None;
        }
        // we are deadlocked
        if self.is_deadlocked() {
            return None;
        }
        let instruction = self.instructions[self.cur];
        // println!("{}: {} - {:?} - {:?}", self.pid, self.cur, instruction, self.registers);
        match instruction {
            Instruction::Snd(x) => {
                self.snd_value = *self.registers.entry(x).or_insert(0);
                self.snd_count += 1;
                self.cur += 1;
            },
            Instruction::Set(x, v) => {
                self.registers.insert(x, v);
                self.cur += 1;
            },
            Instruction::Setr(x, r) => {
                let v = *self.registers.entry(r).or_insert(0);
                self.registers.insert(x, v);
                self.cur += 1;
            },
            Instruction::Add(x, v) => {
                *self.registers.entry(x).or_insert(0) += v;
                self.cur += 1;
            },
            Instruction::Addr(x, r) => {
                let v = *self.registers.entry(r).or_insert(0);
                *self.registers.entry(x).or_insert(0) += v;
                self.cur += 1;
            },
            Instruction::Mul(x, v) => {
                *self.registers.entry(x).or_insert(0) *= v;
                self.cur += 1;
            },
            Instruction::Mulr(x, r) => {
                let v = *self.registers.entry(r).or_insert(0);
                *self.registers.entry(x).or_insert(0) *= v;
                self.cur += 1;
            },
            Instruction::Mod(x, v) => {
                *self.registers.entry(x).or_insert(0) %= v;
                self.cur += 1;
            },
            Instruction::Modr(x, r) => {
                let v = *self.registers.entry(r).or_insert(0);
                *self.registers.entry(x).or_insert(0) %= v;
                self.cur += 1;
            },
            Instruction::Rcv(x) => {
                // only proceed if we're not deadlocked
                if !self.is_deadlocked() {
                    *self.registers.entry(x).or_insert(0) = self.rcv_queue.pop_front().unwrap();
                    self.cur += 1;
                }
            },
            Instruction::Jgz(x, v) => {
                if *self.registers.entry(x).or_insert(0) > 0 {
                    if v < 0 {
                        let abs = v.abs() as usize;
                        self.cur = match self.cur.checked_sub(abs) {
                            Some(n) => n,
                            None => usize::max_value(),
                        }
                    } else {
                        self.cur += v as usize;
                    }
                } else {
                    self.cur += 1;
                }
            },
            Instruction::Jgzr(x, r) => {
                if *self.registers.entry(x).or_insert(0) > 0 {
                    let v = *self.registers.entry(r).or_insert(0);
                    if v < 0 {
                        let abs = v.abs() as usize;
                        self.cur = match self.cur.checked_sub(abs) {
                            Some(n) => n,
                            None => usize::max_value(),
                        }
                    } else {
                        self.cur += v as usize;
                    }
                } else {
                    self.cur += 1;
                }
            },
        };
        Some(self.snd_count)
    }
}

fn from_file(filename: &str) -> Vec<Instruction> {
    let file = File::open(filename).expect("file not found");
    BufReader::new(file).lines()
                        .map(|line| {
                            let line = line.unwrap();
                            let mut splits = line.split(" ");
                            let cmd = splits.next().unwrap();
                            let reg = splits.next().unwrap().parse::<char>().unwrap();
                            let val = splits.next(); // may not exist;
                            let val_val = val.and_then(|v| v.parse::<i64>().ok());
                            let reg_val = val.and_then(|v| v.parse::<char>().ok());
                            match (cmd, val_val, reg_val) {
                                ("snd", _, _) => Instruction::Snd(reg),
                                ("set", Some(v), _) => Instruction::Set(reg, v),
                                ("set", _, Some(r)) => Instruction::Setr(reg, r),
                                ("add", Some(v), _) => Instruction::Add(reg, v),
                                ("add", _, Some(r)) => Instruction::Addr(reg, r),
                                ("mul", Some(v), _) => Instruction::Mul(reg, v),
                                ("mul", _, Some(r)) => Instruction::Mulr(reg, r),
                                ("mod", Some(v), _) => Instruction::Mod(reg, v),
                                ("mod", _, Some(r)) => Instruction::Modr(reg, r),
                                ("rcv", _, _) => Instruction::Rcv(reg),
                                ("jgz", Some(v), _) => Instruction::Jgz(reg, v),
                                ("jgz", _, Some(r)) => Instruction::Jgzr(reg, r),
                                (_, _, _) => panic!("Unhandled: {:?} {:?} {:?}", cmd, val_val, reg_val),
                            }
                        })
                        .collect()
}

fn main() {
    // let filename = "example_2";
    let filename = "question";
    let instructions = from_file(filename);
    let mut sound_card_0 = SoundCard::new(instructions.clone(), 0);
    let mut sound_card_1 = SoundCard::new(instructions, 1);

    // mangage the queue exchanging
    let mut last_sound_card_0_snd_count = 0;
    let mut last_sound_card_1_snd_count = 0;
    // alternate executing sound cards to prevent a memory explosion
    for i in 0..usize::max_value() {
        // sound_card 0
        // println!("{} sound_card_0..", i);
        loop {
            if sound_card_0.is_deadlocked() {
                break;
            }
            if last_sound_card_0_snd_count != sound_card_0.snd_count {
                last_sound_card_0_snd_count = sound_card_0.snd_count;
                sound_card_1.rcv_queue.push_back(sound_card_0.snd_value);
            }
            sound_card_0.next();
        }
        // sound_card 1
        // println!("{} sound_card_1..", i);
        loop {
            if sound_card_1.is_deadlocked() {
                break;
            }
            if last_sound_card_1_snd_count != sound_card_1.snd_count {
                last_sound_card_1_snd_count = sound_card_1.snd_count;
                sound_card_0.rcv_queue.push_back(sound_card_1.snd_value);
            }
            sound_card_1.next();
        }
        // exit if both sound cards are deadlocked
        if sound_card_0.is_deadlocked() && sound_card_1.is_deadlocked() {
            break;
        }
    }
    println!("{}", sound_card_1.snd_count);
}

