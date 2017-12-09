use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

struct Info {
    holds: Vec<String>,
    weight: u32,
}

struct ProgramGraph {
    adj_list: HashMap<String, Info>
}

impl ProgramGraph {

    /// Construct a ProgramGraph from a file.
    fn new(filename: &str) -> ProgramGraph {
        let mut adj_list: HashMap<String, Info> = HashMap::new();
        let file = File::open(filename).expect("file not found");
        for line in BufReader::new(&file).lines() {
            let (name, info) = parse_line(&line.expect("unable to read line"));
            adj_list.insert(name, info);
        }
        ProgramGraph { adj_list }
    }

    /// Return the name of the root program.
    fn root(&self) -> &str {
        self.adj_list.keys()
                     .find(|&name| !self.adj_list.values()
                                                 .flat_map(|info| info.holds.iter())
                                                 .any(|n| name == n))
                     .unwrap()
    }

    /// Return the total weight of a program - the program itself and all its children.
    fn total_weight(&self, name: &str) -> u32 {
        let info = self.adj_list.get(name).unwrap();
        let holds: u32 = info.holds.iter()
                                   .map(|n| self.total_weight(n))
                                   .sum();
        info.weight + holds
    }

    /// Return whether the held programs are unbalanced for a given program.
    fn is_unbalanced(&self, name: &str) -> bool {
        let info = self.adj_list.get(name).unwrap();
        if info.holds.is_empty() {
            return false;
        }
        // ensure the total weight of all held programs are equal
        let held_program = &info.holds[0];
        info.holds.iter()
                  .any(|n| self.total_weight(n) != self.total_weight(&held_program))  
    }

    /// Return all unbalanced programs.
    fn find_unbalanced(&self) -> Vec<&String> {
        self.adj_list.keys()
                     .filter(|&n| self.is_unbalanced(n))
                     .collect()
    }

    /// Return the total weight of held programs that would balance a given unbalanced program.
    fn balanced_held_total_weight(&self, name: &str) -> u32 {
        if !self.is_unbalanced(name) { panic!("program must be unbalanced") }

        // track the total weights seen so far
        // when a duplicate weight is seen, return it
        // otherwise, add it to the array
        let mut held_total_weights: [u32; 2] = [0; 2];
        let mut i = 0;
        for n in self.adj_list.get(name).unwrap().holds.iter() {
            let total_weight = self.total_weight(n);
            if held_total_weights.contains(&total_weight) {
                return total_weight
            }
            held_total_weights[i] = total_weight;
            i += 1;
        }
        panic!("program did not have more than 2 held programs");
    }

    /// The weight required of a held program to rebalance the given unbalanced program.
    fn rebalance_weight(&self, name: &str) -> (&String, u32) {
        if !self.is_unbalanced(name) { panic!("program must be unbalanced") }

        // find the problem held program
        let expected = self.balanced_held_total_weight(name);
        let problem = self.adj_list.get(name).unwrap().holds.iter()
                                                            .find(|&n| self.total_weight(n) != expected)
                                                            .unwrap();
        // calcuated the weight it needs to rebalance its holder
        let info = self.adj_list.get(problem).unwrap();
        let problem_held_weight: u32 = info.holds.iter()
                                                 .map(|n| self.total_weight(n))
                                                 .sum();
        (problem, expected - problem_held_weight)
    }
}

/// Parse line of the ProgramGraph returning information about a program.
fn parse_line(line: &str) -> (String, Info) {
    let name = line.find(char::is_whitespace)
                   .map(|nd| String::from(&line[..nd]))
                   .unwrap();
    let weight = line.find('(')
                     .map(|lp| line.find(')')
                                   .map(|rp| (&line[(lp+1)..rp]).parse::<u32>().unwrap()))
                                   .unwrap()
                     .unwrap();
    let holds = line.find('>')
                    .map(|ad| (&line[(ad+2)..]).split(", ")
                                               .map(String::from)
                                               .collect::<Vec<String>>())
                    .unwrap_or(Vec::new());
    (name, Info { holds: holds, weight: weight})
}

fn main() {
    let filename = "question";
    let program_graph = ProgramGraph::new(filename);

    // find all unbalanced programs
    // the lightest total weight is the problematic program
    let mut unbalanced = program_graph.find_unbalanced();
    unbalanced.sort_by_key(|&n| program_graph.total_weight(n));
    let lightest = unbalanced.first().unwrap();
    println!("unbalanced [{:?}] lightest [{}]", unbalanced, lightest);

    // find the required problem weight to rebalance the program
    let (problem, rebalance_weight) = program_graph.rebalance_weight(lightest);
    println!("problem [{}] rebalance_weight [{}]", problem, rebalance_weight);
}
