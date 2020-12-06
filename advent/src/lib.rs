use regex::{Captures, Regex};

use std::{io::{BufReader, BufRead}, fs::File};
use std::path::{Path, PathBuf};

pub mod grid;

pub trait FromRegex {
    fn from(captures: Captures) -> Self;
}

pub struct InputSnake {
    path: PathBuf,
}

pub struct GroupIterator {
    line_iter: Box<dyn Iterator<Item = String>>,
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

    /// 🔢🐍
    pub fn int_snake(&self) -> Box<dyn Iterator<Item = i64>> {
        Box::new(self.snake().map(|s| s.parse::<i64>().unwrap()))
    }

    /// ✱🐍
    pub fn regex_snake<T: FromRegex>(&self, regex_str: &'static str) -> Box<dyn Iterator<Item = T>> {
        let regex = Regex::new(regex_str).unwrap();
        let items = self.snake()
            .map(move |s| FromRegex::from(regex.captures(&s).unwrap()));
        Box::new(items)
    }

    /// (🐍)
    pub fn group_snake(&self) -> Box<dyn Iterator<Item = Vec<String>>> {
        Box::new(GroupIterator {
            line_iter: Box::new(self.snake())
        })
    }

    /// ❌🐍
    pub fn no_snake(&self) -> String {
        std::fs::read_to_string(&self.path).expect("Unable to open file at path")
            .trim()
            .to_string()
    }
}
