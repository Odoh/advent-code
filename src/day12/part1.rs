use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

type Graph = Vec<Vec<u32>>;

fn from_file(filename: &str) -> Graph {
    let mut graph: Vec<Vec<u32>> = vec!();

    let file = File::open(filename).expect("file not found");
    for line in BufReader::new(&file).lines()
                                     .filter_map(Result::ok) {
        let pipes = line.find('>')
                        .map(|d| (&line[(d+2)..]).split(", ")
                                                 .map(|id| id.parse::<u32>().unwrap())
                                                 .collect::<Vec<u32>>())
                        .unwrap_or(Vec::new());
        graph.push(pipes);
    }
    graph
}

fn connected_nodes(graph: &Graph, id: u32, seen_nodes: &mut HashSet<u32>) {
    // if already seen, skip processing
    if seen_nodes.contains(&id) {
        return;
    }

    // insert outselves and our connected nodes, recursively
    seen_nodes.insert(id);
    for connected_node in graph[id as usize].iter() {
        connected_nodes(graph, *connected_node, seen_nodes);
    }
}

fn main() {
    // let graph = from_file("example");
    let graph = from_file("question");

    let mut nodes: HashSet<u32> = HashSet::new();
    connected_nodes(&graph, 0, &mut nodes);

    // println!("graph: {:?}", graph);
    // println!("nodes: {:?}", nodes);
    println!("{}", nodes.len());
}
