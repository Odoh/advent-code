
use std::collections::HashSet;

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

fn dance_old(programs: &mut Programs, moves: &str) {
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

#[derive(Debug)]
enum Move {
    Spin(usize),
    SwapPos(usize, usize),
    SwapName(char, char),
}

fn from_moves(moves: &str) -> Vec<Move> {
    moves.split(',')
         .map(|mv| {
        let mv_type = mv.chars().next().unwrap();
        match mv_type {
            's' => {
                let size = &mv[1..].parse::<usize>().unwrap();
                Move::Spin(*size)
            },
            'x' => {
                let indices: Vec<usize> = (&mv[1..]).split('/')
                                                    .map(|s| s.parse::<usize>().unwrap())
                                                    .collect();
                Move::SwapPos(indices[0], indices[1])
            },
            'p' => {
                let names: Vec<char> = (&mv[1..]).split('/')
                                                 .map(|s| s.parse::<char>().unwrap())
                                                 .collect();
                Move::SwapName(names[0], names[1])
            },
             _ => panic!("Unsupported move char {}", mv_type),
        }
    }).collect()
}

fn spin_to_swap(programs_size: usize, moves: Vec<Move>) -> Vec<Move> {
    let mut vec = Vec::new();
    for mv in moves.into_iter() {
        match mv {
            Move::Spin(size) => {
                for _ in 0..size {
                    for i in 0..programs_size {
                        vec.push(Move::SwapPos(0, (i + 1) % programs_size))
                    }
                }
            }
            _ => vec.push(mv)
        }
    }
    vec
}

fn reduce_swap_pos(programs_size: usize, moves: Vec<Move>) -> Vec<Move> {
    let mut vec = Vec::new();
    let mut pos: Vec<usize> = Vec::new();
    for mv in moves.into_iter() {
        match mv {
            Move::SwapPos(p1, p2) => {
                if pos.is_empty() {
                    pos = (0..programs_size).collect();
                }
                pos.swap(p1, p2);
            },
            _ => {
                let mut new_pos: Vec<usize> = (0..programs_size).collect();
                for (i, &p) in pos.iter().enumerate() {
                    if new_pos[i] != p {
                        let j = new_pos.iter().position(|&v| v == p).unwrap();
                        new_pos.swap(i, j);
                        vec.push(Move::SwapPos(i, j));
                    }
                }
                pos.clear();
                vec.push(mv);
            }
        }
    }
    if !pos.is_empty() {
        let mut new_pos: Vec<usize> = (0..programs_size).collect();
        for (i, &p) in pos.iter().enumerate() {
            if new_pos[i] != p {
                let j = new_pos.iter().position(|&v| v == p).unwrap();
                new_pos.swap(i, j);
                vec.push(Move::SwapPos(i, j));
            }
        }
    }
    vec
}

fn dance(programs: &mut Programs, moves: &Vec<Move>) {
    for mv in moves.iter() {
        match mv {
            &Move::Spin(size) => programs.spin(size),
            &Move::SwapPos(p1, p2) => programs.swap_indices(p1, p2),
            &Move::SwapName(n1, n2) => programs.swap_programs(n1, n2),
        }
    }
}

fn main() {
    // let example = "abcde";
    let question = "abcdefghijklmnop";
    let moves_str = include_str!("question").trim();
    // let moves_str = "s2,x3/4,pe/b";
    // let moves_str = "s2";
    // let moves_str = "x0/1,x1/2";

    // old
    let mut programs_old = Programs::new(question);
    dance_old(&mut programs_old, moves_str);
    println!("{}", programs_old.to_string());

    // new
    let mut programs = Programs::new(question);
    let moves = from_moves(moves_str);
    // println!("{:?}", moves);

    let no_spin_moves = spin_to_swap(question.len(), moves); 
    // println!("{:?}", no_spin_moves);

    let reduce_swap_moves = reduce_swap_pos(question.len(), no_spin_moves);
    // println!("{:?}", reduce_swap_moves);
    
    // repeats every 60 dances
    let mut seen_programs: HashSet<String> = HashSet::new();
    for i in 0..(1_000_000_000 % 60) {
        dance(&mut programs, &reduce_swap_moves);
        let ps = programs.to_string();
        if !seen_programs.insert(ps.clone()) {
            println!("program was already seen {} {}", i, ps);
        }
    }

    println!("{}", programs.to_string());
}
