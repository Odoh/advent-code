use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn parse_line(line: &str) -> (String, Vec<String>) {
    let nd = line.find(char::is_whitespace).unwrap();
    let name = String::from(&line[..nd]);
    let holds = line.find('>')
                    .map(|ad| (&line[(ad+2)..]).split(", ")
                                               .map(String::from)
                                               .collect::<Vec<String>>())
                    .unwrap_or(Vec::new());
    (name, holds)
}

fn parse_file(filename: &str) -> HashMap<String, Vec<String>> {
    // build adj_list of graph from the file
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    let file = File::open(filename).expect("file not found");
    for line in BufReader::new(&file).lines() {
        let (name, holds) = parse_line(&line.expect("unable to read line"));
        adj_list.insert(name, holds);
    }
    return adj_list;
}

fn main() {
    let adj_list = parse_file("question");

    // find the root of the root of the directed graph
    // which is the key which is not being held by any other key
    let root = adj_list.keys()
                       .find(|&name| !adj_list.values()
                                              .flat_map(|holds| holds.iter())
                                              .any(|n| name == n))
                       .unwrap();
    println!("{}", root);
}
