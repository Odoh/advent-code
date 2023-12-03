use std::collections::{HashMap, HashSet};
use pancurses::Window;
use log::debug;

pub use pancurses::Input;
pub type Coord = (i64, i64);

pub const DRAW_CHAR: fn (Coord, Option<&char>) -> Option<char> = |_, c| c.map(|&c| c);

pub struct Grid<T: std::fmt::Debug> {
    coord_to_entry: HashMap<Coord, T>,
    not_drawn_coords: HashSet<Coord>,
    min_x: i64, max_x: i64,
    min_y: i64, max_y: i64,
    window: Option<Window>,
    input_window: Option<Window>,
}

pub enum DrawType<'a> {
    Curses(&'a Window),
    StdOut,
}

const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownLeft,
    Direction::DownRight,
];

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub struct GridIterator<'a, T> 
    where T: std::fmt::Debug {

    grid: &'a Grid<T>,
    coord: Coord,
}

pub struct GridCoordinateIterator<'a, T>
    where T: std::fmt::Debug {

    grid_iter: GridIterator<'a, T>,
}

impl <'a, T> GridIterator<'a, T>
    where T: std::fmt::Debug {

    pub fn new(grid: &'a Grid<T>) -> GridIterator<'a, T> {
        GridIterator {
            grid,
            coord: grid.min_xy(),
        }
    }

    pub fn coords(self) -> GridCoordinateIterator<'a, T> {
        GridCoordinateIterator {
            grid_iter: self,
        }
    }
}

impl Direction {
    pub fn next_coord(self, coord: Coord) -> Coord {
        self.next_coord_by_value(coord, 1)
    }

    pub fn next_coord_by_value(self, coord: Coord, value: i64) -> Coord {
        match self {
            Direction::Left => (coord.0 - value, coord.1),
            Direction::Right => (coord.0 + value, coord.1),
            Direction::Up => (coord.0, coord.1 - value),
            Direction::Down => (coord.0, coord.1 + value),
            Direction::UpLeft => (coord.0 - value, coord.1 - value),
            Direction::UpRight => (coord.0 + value, coord.1 - value),
            Direction::DownLeft => (coord.0 - value, coord.1 + value),
            Direction::DownRight => (coord.0 + value, coord.1 + value),
        }
    }
}

impl <'a> PartialEq for DrawType<'a> {
    fn eq(&self, other: &DrawType) -> bool {
        match (self, other) {
            (DrawType::Curses(_), DrawType::Curses(_)) => true,
            (DrawType::StdOut, DrawType::StdOut) => true,
            _ => false,
        }
    }
}

pub struct ReadProxy {
    input_window: Window,
}

impl <T: std::fmt::Debug> Grid<T> {
    pub fn new() -> Grid<T> {
        Grid {
            coord_to_entry: HashMap::new(),
            not_drawn_coords: HashSet::new(),
            min_x: 0, max_x: 0,
            min_y: 0, max_y: 0,
            window: None,
            input_window: None,
        }
    }

    pub fn entry(&self, coord: Coord) -> Option<&T> {
        self.coord_to_entry.get(&coord)
    }

    pub fn entries(&self) -> Vec<&T> {
        self.coord_to_entry.values().collect()
    }

    pub fn coord_entries(&self) -> Vec<(Coord, &T)> {
        self.coord_to_entry.iter().map(|(c, t)| (*c, t)).collect()
    }

    pub fn directional_entries(&self, mut coord: Coord, direction: Direction) -> Vec<&T> {
        let mut entries = Vec::new();
        loop {
            coord = direction.next_coord(coord);
            if let Some(entry) = self.entry(coord) {
                entries.push(entry);
            } else {
                return entries;
            }
        }
    }

    pub fn all_directional_entries(&self, coord: Coord) -> Vec<Vec<&T>> {
        ALL_DIRECTIONS.iter()
            .map(|&direction| self.directional_entries(coord, direction))
            .collect()
    }

    pub fn adjacent_entries(&self, coord: Coord) -> Vec<&T> {
        ALL_DIRECTIONS.iter()
            .filter_map(|direction| self.entry(direction.next_coord(coord)))
            .collect()
    }

    pub fn adjacent_coord_entries(&self, coord: Coord) -> Vec<(Coord, &T)> {
        ALL_DIRECTIONS.iter()
            .filter_map(|direction| {
                let next_coord = direction.next_coord(coord);
                let entry = self.entry(next_coord);
                entry.map(|e| (next_coord, e))
            })
            .collect()
    }

    pub fn min_xy(&self) -> (i64, i64) {
        (self.min_x, self.min_y)
    }

    pub fn max_xy(&self) -> (i64, i64) {
        (self.max_x, self.max_y)
    }

    pub fn add_entry(&mut self, coord: Coord, entry: T) {
        if coord.0 < self.min_x {
            self.min_x = coord.0;
        } else if coord.0 > self.max_x {
            self.max_x = coord.0;
        }

        if coord.1 < self.min_y {
            self.min_y = coord.1;
        } else if coord.1 > self.max_y {
            self.max_y = coord.1;
        }

        debug!("Add entry: {:?} at {:?}", entry, coord);
        self.coord_to_entry.insert(coord, entry);
        self.not_drawn_coords.insert(coord);
    }

    pub fn init_curses(&mut self) {
        let stdscr = pancurses::initscr();
        self.window = Some(stdscr);
    }

    pub fn init_curses_input(&mut self, x: i32, y: i32) {
        let input_window = pancurses::newwin(1, 1, x, y);
        input_window.nodelay(true);
        input_window.keypad(true);
        self.input_window = Some(input_window);
    }

    pub fn end_curses(&mut self) {
        if let Some(_) = self.window.take() {
            pancurses::endwin();
        }
    }

    pub fn draw(&mut self, entry_char: fn (Coord, Option<&T>) -> Option<char>) {
        for x in self.min_x..=self.max_x {
            for y in self.min_y..=self.max_y {
                let coord: Coord = (x, y);

                if self.draw_type() == DrawType::StdOut {
                    let entry = self.coord_to_entry.get(&coord);
                    if let Some(c) = entry_char(coord, entry) {
                        print!("{}", c);
                    } else {
                        print!(" ");
                    }
                    continue;
                }

                if self.not_drawn_coords.contains(&coord) {
                    self.not_drawn_coords.remove(&coord);

                    if let DrawType::Curses(window) = self.draw_type() {
                        let entry = self.coord_to_entry.get(&coord);
                        if let Some(c) = entry_char(coord, entry) {
                            window.mvaddch(y as i32, x as i32, c);
                            window.refresh();
                        }
                    }
                }
            }
            if self.draw_type() == DrawType::StdOut {
                println!();
            }
        }
    }

    pub fn draw_type(&self) -> DrawType {
        if self.window.is_some() {
            DrawType::Curses(self.window.as_ref().unwrap())
        } else {
            DrawType::StdOut
        }
    }

    pub fn read_key(&self) -> Option<Input> {
        self.input_window.as_ref().expect("Input window has been taken").getch()
    }

    pub fn read_proxy(&mut self) -> ReadProxy {
        ReadProxy::new(self.input_window.take().expect("Input window was already taken"))
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator::new(self)
    }
}


const ITER_COMPLETE: Coord = (-1, -1);

impl <'a, T: std::fmt::Debug> Iterator for GridIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.coord == ITER_COMPLETE {
            return None
        }

        let (_min_x, min_y) = self.grid.min_xy();
        let (max_x, max_y) = self.grid.max_xy();
        let item = self.grid.entry(self.coord);

        let (next_x, next_y) = if self.coord.0 == max_x && self.coord.1 == max_y {
            ITER_COMPLETE
        } else if self.coord.1 + 1 <= max_y {
            (self.coord.0, self.coord.1 + 1)
        } else {
            (self.coord.0 + 1, min_y)
        };

        self.coord = (next_x, next_y);
        item
    }
}

impl <'a, T: std::fmt::Debug> Iterator for GridCoordinateIterator<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<(Coord, &'a T)> {
        let coord = self.grid_iter.coord;
        let item = self.grid_iter.next();

        item.map(|i| (coord, i))
    }
}

impl ReadProxy {
    pub fn new(input_window: Window) -> ReadProxy {
        ReadProxy {
            input_window,
        }
    }

    pub fn read_key(&self) -> Option<Input> {
        self.input_window.getch()
    }
}
