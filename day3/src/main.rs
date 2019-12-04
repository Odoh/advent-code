use std::collections::{HashMap, HashSet};
use advent::InputSnake;

const CENTRAL_PORT: Point = (0, 0);

type WireId = usize;
type Point = (i32, i32);

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Direction {
    pub fn from(s: &str) -> Self {
        match s.chars().next() {
            Some('U') => Direction::Up(s.split_at('U'.len_utf8()).1.parse::<i32>().expect("Up direction to be an i32")),
            Some('D') => Direction::Down(s.split_at('D'.len_utf8()).1.parse::<i32>().expect("Down direction to be an i32")),
            Some('L') => Direction::Left(s.split_at('L'.len_utf8()).1.parse::<i32>().expect("Left direction to be an i32")),
            Some('R') =>Direction::Right(s.split_at('R'.len_utf8()).1.parse::<i32>().expect("Right direction to be an i32")),
            Some(_) | None => panic!("Unhandled Direction"),
        }
    }
}

struct FrontPanel {
    wire_to_path: HashMap<WireId, Vec<Point>>,
    overlaps: HashMap<Point, HashSet<WireId>>,
}

impl FrontPanel {
    pub fn new() -> Self {
        FrontPanel {
            wire_to_path: HashMap::new(),
            overlaps: HashMap::new(),
        }
    }

    pub fn add_wire(&mut self, wire_id: WireId, directions: &[Direction]) {
        let mut point = CENTRAL_PORT;
        for direction in directions {
            let mut path = FrontPanel::direction_to_path(&point, direction);
            point = path.pop().expect("Direction path does not contain elements");

            path.iter().for_each(|p| self.handle_overlap(wire_id, p));

            let wire_path = self.wire_to_path.entry(wire_id).or_insert(Vec::new());
            wire_path.extend(path);
        }
    }

    pub fn closest_overlap_manhattan(&self) -> i32 {
        let manhattan_distance = |p: &Point| (CENTRAL_PORT.0 - p.0).abs() + (CENTRAL_PORT.1 - p.1).abs();
        self.overlaps.keys()
            .filter(|&p| *p != CENTRAL_PORT)
            .min_by_key(|p| manhattan_distance(p))
            .map(|p| manhattan_distance(p))
            .expect("Expect at least one overlap point")
    }

    pub fn closest_overlap_steps(&self) -> usize {
        let step_distance = |point: &Point, wires: &HashSet<WireId>| wires.iter()
            .map(|wid| self.wire_to_path.get(wid).expect("Wire to have a path"))
            .map(|path| path.iter().position(|&p| p == *point).expect("Expect point to be in the path"))
            .sum();
        self.overlaps.iter()
            .filter(|(&point, _)| point != CENTRAL_PORT)
            .min_by_key(|(point, wires)| step_distance(point, wires))
            .map(|(point, wires)| step_distance(point, wires))
            .expect("Expect at least one overlap point")
    }

    fn handle_overlap(&mut self, wire_id: WireId, point: &Point) {
        for (wid, points) in self.wire_to_path.iter() {
            if *wid == wire_id {
                continue;
            }
            if points.contains(point) {
                let wires = self.overlaps.entry(*point).or_insert(HashSet::new());
                wires.insert(*wid);
                wires.insert(wire_id);

                // can return early as other wires will already be added to the overlap set
                return;
            }
        }
    }

    fn direction_to_path(init_point: &Point, direction: &Direction) -> Vec<Point> {
        match direction {
            Direction::Up(y) => (init_point.1..=(init_point.1 + y))
                .map(|y| (init_point.0, y))
                .collect(),
            Direction::Down(y) => ((init_point.1 - y)..=init_point.1).rev()
                .map(|y| (init_point.0, y))
                .collect(),
            Direction::Left(x) => ((init_point.0 - x)..=init_point.0).rev()
                .map(|x| (x, init_point.1))
                .collect(),
            Direction::Right(x) => (init_point.0..=(init_point.0 + x))
                .map(|x| (x, init_point.1))
                .collect(),
        }
    }
}

fn parse_path_string(path_str: &str) -> Vec<Direction> {
    path_str.split(',')
        .map(|dir_str| Direction::from(&dir_str))
        .collect::<Vec<Direction>>()
}

fn part_one() {
    let input = InputSnake::new("input");
    let paths = input.snake()
        .map(|path_str| parse_path_string(&path_str))
        .collect::<Vec<Vec<Direction>>>();

    let mut front_panel = FrontPanel::new();
    paths.iter()
        .enumerate()
        .for_each(|(i, path)| front_panel.add_wire(i, &path));

    let closest_overlap_manhattan = front_panel.closest_overlap_manhattan();
    println!("Part One: {:?}", closest_overlap_manhattan);
}

fn part_two() {
    let input = InputSnake::new("input");
    let paths = input.snake()
        .map(|path_str| parse_path_string(&path_str))
        .collect::<Vec<Vec<Direction>>>();

    let mut front_panel = FrontPanel::new();
    paths.iter()
        .enumerate()
        .for_each(|(i, path)| front_panel.add_wire(i, &path));

    let closest_overlap_steps = front_panel.closest_overlap_steps();
    println!("Part Two: {:?}", closest_overlap_steps)
}

fn main() {
    part_one();
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let paths1 = parse_path_string("R8,U5,L5,D3");
        let paths2 = parse_path_string("U7,R6,D4,L4");

        let mut front_panel = FrontPanel::new();
        front_panel.add_wire(0, &paths1);
        front_panel.add_wire(1, &paths2);
    }

    #[test]
    fn test_part_2() {
        let paths1 = parse_path_string("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let paths2 = parse_path_string("U62,R66,U55,R34,D71,R55,D58,R83");

        let mut front_panel = FrontPanel::new();
        front_panel.add_wire(0, &paths1);
        front_panel.add_wire(1, &paths2);

        println!("{:?}", front_panel.closest_overlap_steps());
    }
}
