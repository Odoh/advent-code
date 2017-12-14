use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::Ordering;

/// Read a file of scanner locations.
fn from_file(filename: &str) -> Vec<usize> {
    let mut layers: Vec<usize> = Vec::new();

    // file indicates the layers and depths that are scannable
    let file = File::open(filename).expect("file not found");
    for line in BufReader::new(&file).lines().filter_map(Result::ok) {
        let d = line.find(':').unwrap();
        let layer = (&line[..d]).parse::<usize>().unwrap();
        let depth = (&line[(d+2)..]).parse::<usize>().unwrap();

        while layers.len() < layer {
            let missing_layer = layers.len();
            layers.push(0);
        }
        layers.push(depth);
    }
    layers
}

/// Return the severity of a trip through the firewall.
fn severity(layers: &Vec<usize>) -> usize {
    let mut severity = 0;
    let mut packet = 0;
    for time in (0..usize::max_value()) {
        // packet made it through
        if packet == layers.len() {
            break;
        }

        // no scanner at this layer
        if layers[packet] == 0 {
            packet += 1;
            continue
        }

        // is scanner at depth == 1 in this layer?
        if (time % (2 * layers[packet] - 2)) == 0 {
            severity += packet * layers[packet];
        }

        packet += 1;
    }
    return severity;
}

fn main() {
    // let layers = from_file("example");
    let layers = from_file("question");
    let severity = severity(&layers);
    println!("{}", severity);
}
