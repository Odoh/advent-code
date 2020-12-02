use regex::{Captures, Regex};

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};

pub mod grid;

pub trait FromRegex {
    fn from(captures: Captures) -> Self;
}

pub struct InputSnake {
    path: PathBuf,
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
        return Box::new(f.lines().map(|l| l.unwrap()))
    }

    /// 🔢🐍
    pub fn int_snake(&self) -> Box<dyn Iterator<Item = i64>> {
        return Box::new(self.snake().map(|s| s.parse::<i64>().unwrap()));
    }

    /// ✱🐍
    pub fn regex_snake<T: FromRegex>(&self, regex_str: &'static str) -> Box<dyn Iterator<Item = T>> {
        let regex = Regex::new(regex_str).unwrap();
        let items = self.snake()
            .map(move |s| FromRegex::from(regex.captures(&s).unwrap()));
        return Box::new(items);
    }

    /// ❌🐍
    pub fn no_snake(&self) -> String {
        return std::fs::read_to_string(&self.path).expect("Unable to open file at path")
            .trim()
            .to_string();
    }
}
