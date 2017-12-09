struct JumpMaze {
    jump_offsets: Vec<i32>,
}

impl JumpMaze {
    fn new(jump_offsets: Vec<i32>) -> Self {
        JumpMaze { jump_offsets }
    }
}

impl IntoIterator for JumpMaze {
    type Item = i32;
    type IntoIter = JumpMazeIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        JumpMazeIntoIterator { jump_maze: self,
                               pos: 0 }
    }
}

struct JumpMazeIntoIterator {
    jump_maze: JumpMaze,
    pos: i32,
}

impl Iterator for JumpMazeIntoIterator {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < 0 {
            return None;
        }
        let index = self.pos as usize;
        if index >= self.jump_maze.jump_offsets.len() {
            return None;
        }
        let jump = self.jump_maze.jump_offsets[index];
        self.jump_maze.jump_offsets[index] += if jump >= 3 { -1 } else { 1 };
        self.pos += jump;
        Some(jump)
    }
}

#[cfg(test)]
mod test {
    use super::JumpMaze;

    #[test]
    fn example() {
        let jump_offsets = vec![0, 3, 0, 1, -3];
        let jump_maze = JumpMaze::new(jump_offsets);
        let steps = jump_maze.into_iter().count();
        assert_eq!(steps, 10);
    }
}

fn main() {
    let jump_str: &str = include_str!("question");
    let jump_offsets: Vec<i32> = jump_str.split_whitespace()
                                         .into_iter()
                                         .map(|s| s.parse::<i32>())
                                         .filter_map(Result::ok)
                                         .collect();
    let jump_maze = JumpMaze::new(jump_offsets);
    let steps = jump_maze.into_iter().count();
    println!("{}", steps);
}