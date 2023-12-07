use regex::{CaptureMatches, Regex};
use nom::{Parser, error::ParseError};

use std::{io::{BufReader, BufRead}, fs::File};
use std::path::{Path, PathBuf};
use std::marker::PhantomData;

pub mod grid;
pub mod map;
pub mod snake;

pub trait FromRegex {
    fn from(capture_matches: CaptureMatches) -> Self;
}

pub struct InputSnake {
    path: PathBuf,
}

pub struct GroupIterator {
    line_iter: Box<dyn Iterator<Item = String>>,
}

pub struct NomOutput<P, O> {
    pub line: String,
    pub parser: P,
    _phantom: PhantomData<O>,
}

impl <P, O> NomOutput<P, O> 
    where
        P: for<'a> nom::Parser<&'a str, O, nom::error::Error<&'a str>> {

    pub fn new(line: String, parser: P) -> Self {
        NomOutput::<P, O> {
            line,
            parser,
            _phantom: Default::default(),
        }
    }

   pub fn parse(&mut self) -> O {
        self.parser.parse(&self.line).unwrap().1
   }
}

impl GroupIterator {
    pub fn new(line_iter: Box<dyn Iterator<Item = String>>) -> Self {
        GroupIterator {
            line_iter,
        }
    }
}

impl Iterator for GroupIterator {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut group = Vec::new();
        loop {
            let line = match self.line_iter.next() {
                None => {
                    return if group.is_empty() {
                        None
                    } else {
                        Some(group)
                    };
                }
                Some(line) => line,
            };

            if line.is_empty() {
                return Some(group);
            }

            group.push(line)
        }
    }
}

impl InputSnake {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        return InputSnake {
            path: path.as_ref().to_path_buf()
        }
    }

    /// 🐍
    pub fn snake(&self) -> Box<dyn Iterator<Item = String>> {
        let f = File::open(&self.path).expect("Unable to open file at path");
        let f = BufReader::new(f);
        Box::new(f.lines().map(|l| l.unwrap()))
    }

    /// 🍕🐍
    pub fn nom_snake<P, O>(&self, parser: P) -> Box<dyn Iterator<Item = NomOutput<P, O>>>
    where
        P: for<'a> nom::Parser<&'a str, O, nom::error::Error<&'a str>> + 'static + Copy {

        Box::new(self.snake().map(move |line| NomOutput::new(line, parser)))
    }

    /// 🔢🐍
    pub fn int_snake(&self) -> Box<dyn Iterator<Item = i64>> {
        Box::new(self.snake().map(|s| s.parse::<i64>().unwrap()))
    }

    /// ✱🐍
    pub fn regex_snake<T: FromRegex>(&self, regex_str: &'static str) -> Box<dyn Iterator<Item = T>> {
        let regex = Regex::new(regex_str).unwrap();
        let items = self.snake()
            .map(move |s| FromRegex::from(regex.captures_iter(&s)));
        Box::new(items)
    }

    /// (🐍)
    pub fn group_snake(&self) -> Box<dyn Iterator<Item = Vec<String>>> {
        Box::new(GroupIterator {
            line_iter: Box::new(self.snake())
        })
    }

    /// 🐍🐍
    /// 🐍🐍
    pub fn grid_snake(&self) -> grid::Grid<char> {
        let mut grid= grid::Grid::new();
        self.snake()
            .enumerate()
            .for_each(|(x, line)| line.chars().enumerate()
                .for_each(|(y, c)|
                    grid.add_entry((x as i64, y as i64), c)));
        grid
    }

    /// ❌🐍
    pub fn no_snake(&self) -> String {
        std::fs::read_to_string(&self.path).expect("Unable to open file at path")
            .trim()
            .to_string()
    }
}
