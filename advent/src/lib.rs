use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};

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

    /// ❌🐍
    pub fn no_snake(&self) -> String {
        return std::fs::read_to_string(&self.path).expect("Unable to open file at path")
            .trim()
            .to_string();
    }
}

