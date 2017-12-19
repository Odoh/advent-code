use std::io::{BufRead, BufReader};
use std::fs::File;

const LETTERS: [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Path {
    Vertical,
    Horizontal,
    Junction,
    Letter(char),
    Empty,
}

#[derive(Debug)]
struct Diagram {
    grid: Vec<Vec<Path>>,
    start: (usize, usize),
}

struct PathIter<'a> {
    diagram: &'a Diagram,
    pos: (usize, usize),
    dir: Direction,
}

impl<'a> Iterator for PathIter<'a> {
    type Item = Path;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.pos;
        let up: Option<Path> = x.checked_sub(1)
                                .and_then(|_| self.diagram.grid.get(x - 1))
                                .and_then(|v| v.get(y))
                                .map(|p| *p);
        let down: Option<Path> = self.diagram.grid.get(x + 1)
                                                  .and_then(|v| v.get(y))
                                                  .map(|p| *p);
        let left: Option<Path> = y.checked_sub(1)
                                  .and_then(|_| self.diagram.grid.get(x))
                                  .and_then(|v| v.get(y - 1))
                                  .map(|p| *p);
        let right: Option<Path> = self.diagram.grid.get(x)
                                                   .and_then(|v| v.get(y + 1))
                                                   .map(|p| *p);
        let (next_dir, next_path) = match (self.dir, up, down, left, right) {
            (Direction::Up, Some(u), _, _, _) if u != Path::Empty => (Direction::Up, u),
            (Direction::Up, _, _, Some(l), _) if l != Path::Empty => (Direction::Left, l),
            (Direction::Up, _, _, _, Some(r)) if r != Path::Empty => (Direction::Right, r),
            (Direction::Down, _, Some(d), _, _) if d != Path::Empty => (Direction::Down, d),
            (Direction::Down, _, _, Some(l), _) if l != Path::Empty => (Direction::Left, l),
            (Direction::Down, _, _, _, Some(r)) if r != Path::Empty => (Direction::Right, r),
            (Direction::Left, _, _, Some(l), _) if l != Path::Empty => (Direction::Left, l),
            (Direction::Left, Some(u), _, _, _) if u != Path::Empty => (Direction::Up, u),
            (Direction::Left, _, Some(d), _, _) if d != Path::Empty => (Direction::Down, d),
            (Direction::Right, _, _, _, Some(r)) if r != Path::Empty => (Direction::Right, r),
            (Direction::Right, Some(u), _, _, _) if u != Path::Empty => (Direction::Up, u),
            (Direction::Right, _, Some(d), _, _) if d != Path::Empty => (Direction::Down, d),
            _ => return None, // reached the end
        };
        let next_pos = match next_dir {
            Direction::Up => (x - 1, y),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y - 1),
            Direction::Right => (x, y + 1),
        };
        self.pos = next_pos;
        self.dir = next_dir;
        Some(next_path)
    }
}

impl Diagram {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<Path>> = Vec::new();
        let file = File::open(input).expect("file not found");
        for line in BufReader::new(file).lines()
                                        .filter_map(Result::ok) {
            let paths: Vec<Path> = line.chars()
                                    .map(|c| match c {
                                        '|' => Path::Vertical,
                                        '-' => Path::Horizontal,
                                        '+' => Path::Junction,
                                        l if LETTERS.contains(&l) => Path::Letter(l),
                                        _ => Path::Empty,
                                    })
                                    .collect();
            grid.push(paths);
        }
        let start_y = grid[0].iter().position(|&p| p == Path::Vertical).unwrap();
        Diagram {
            grid,
            start: (0, start_y),
        }
    }

    fn iter(&self) -> PathIter {
        PathIter {
            diagram: self,
            pos: self.start,
            dir: Direction::Down, 
        }
    }
}

fn main() {
    // let filename = "example";
    let filename = "question";
    let diagram = Diagram::new(filename);
    // println!("{:?}", diagram);
    // for path in diagram.iter() {
        // println!("{:?}", path);
    // }
    let route: String = diagram.iter()
                               .filter_map(|p| match p {
                                   Path::Letter(c) => Some(c),
                                   _ => None
                               })
                               .collect();
    println!("{}", route);
}
