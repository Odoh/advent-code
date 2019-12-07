use advent::InputSnake;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const CENTER_OF_MASS: &str = "COM";

struct Graph {
    adj_map: HashMap<String, Vec<String>>,
    vertices: HashSet<String>,
}

impl Graph {
    pub fn new(edges: &[(String, String)]) -> Self {
        let mut adj_map = HashMap::new();
        let mut vertices = HashSet::new();
        edges.iter().for_each(|(src, dst)| {
            adj_map.entry(src.to_string()).or_insert(Vec::new()).push(dst.to_string());
            vertices.insert(src.to_string());
            vertices.insert(dst.to_string());
        });

        Graph {
            adj_map,
            vertices,
        }
    }

    pub fn vertices(&self) -> HashSet<String> {
        self.vertices.clone()
    }

    pub fn paths(&self, src: &str, dst: &str) -> Vec<Vec<String>> {
        self.bfs_path_recur(src, dst, Vec::new())
    }

    fn bfs_path_recur(&self, src: &str, dst: &str, mut path: Vec<String>) -> Vec<Vec<String>> {
        path.push(src.to_string());

        if src == dst {
            return vec![path];
        }

        self.adj_map.get(src).unwrap()
            .iter()
            .flat_map(|v| self.bfs_path_recur(v, dst, path.clone()))
            .collect()
    }
}

fn parse_edges(input: InputSnake) -> Vec<(String, String)> {
    input.snake()
        .map(|l| l.split(')')
            .map(|s| s.to_string())
            .next_tuple()
            .unwrap())
        .map(|(dst, src)| (src, dst))
        .collect()
}

fn orbital_transfers(src_paths: &[String], dst_paths: &[String]) -> usize {
    for i in 0..src_paths.len() {
        if *(&dst_paths[..].ends_with(&src_paths[i..])) {
            return i - 1  // don't include transfer from src to first node
        }
    }
    panic!("Should have found a common path");
}

fn part_one() {
    let input = InputSnake::new("input");
    let edges = parse_edges(input);
    let graph = Graph::new(&edges[..]);

    let orbits: usize = graph.vertices()
        .iter()
        .filter(|v| *v != CENTER_OF_MASS)
        .map(|k| graph.paths(k, CENTER_OF_MASS))
        .map(|paths| {
            paths.iter()
                .map(|path| path.len() - 1)  // don't include the src vertex
                .sum()
        })
        .map(|v: usize| v)  // ??
        .sum();
    println!("Part One: {:?}", orbits);
}

fn part_two() {
    let input = InputSnake::new("input");
    let edges = parse_edges(input);
    let graph = Graph::new(&edges[..]);

    let path_you = graph.paths("YOU", CENTER_OF_MASS).into_iter().next().expect("expect a path from YOU");
    let path_santa = graph.paths("SAN", CENTER_OF_MASS).into_iter().next().expect("expect a path from SAN");

    let transfers_you = orbital_transfers(&path_you, &path_santa);
    let transfers_santa = orbital_transfers(&path_santa, &path_you);
    let transfers = transfers_you + transfers_santa;

    println!("Part Two: {:?}", transfers);
}

fn main() {
    part_one();
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
    }
}
