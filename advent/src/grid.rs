use std::collections::{HashMap, HashSet};
use pancurses::Window;
use log::debug;

pub use pancurses::Input;
pub type Coord = (i64, i64);

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

                if self.not_drawn_coords.contains(&coord) {
                    self.not_drawn_coords.remove(&coord);

                    if let DrawType::Curses(window) = self.draw_type() {
                        let entry = self.coord_to_entry.get(&coord);
                        if let Some(c) = entry_char(coord, entry) {
                            window.mvaddch(y as i32, x as i32, c);
                            window.refresh();
                        }
                    }
                    continue;
                }

                if self.draw_type() == DrawType::StdOut {
                    let entry = self.coord_to_entry.get(&coord);
                    if let Some(c) = entry_char(coord, entry) {
                        print!("{}", c);
                    } else {
                        print!(" ");
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
