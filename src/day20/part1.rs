use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Particle {
    num: usize,
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
    acc: (i64, i64, i64),
}

fn from_file(filename: &str) -> Vec<Particle> {
    let file = File::open(filename).expect("file not found");
    BufReader::new(file).lines()
                        .filter_map(Result::ok)
                        .enumerate()
                        .map(|(num, line)| {
                            let pos = parse_xyz(&line, "p=<");
                            let vel = parse_xyz(&line, "v=<");
                            let acc = parse_xyz(&line, "a=<");
                            Particle { num, pos, vel, acc }})
                        .collect::<Vec<Particle>>()
}

fn parse_xyz(line: &str, start: &str) -> (i64, i64, i64) {
    // p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
    let vec = line.find(start)
                  .map(|s| {
                      // want the point after the start delimeter
                      let start = s + start.len();
                      (&line[start..]).find('>')
                                      .map(|e| {
                                           let end = e + start;
                                           (&line[start..end]).trim()
                                                              .split(",")
                                                              .map(|v| v.parse::<i64>().unwrap())
                                                              .collect::<Vec<i64>>()})
                                      .unwrap()
                  }).unwrap();
    (vec[0], vec[1], vec[2])
}

fn main() {
    // let input = "example";
    let input = "question";
    let particles = from_file(input);

    // long-term farthest away has the smallest acc
    let slowest_particle = particles.iter()
                                    .min_by_key(|p| {
                                        let (x, y, z) = p.acc;
                                        x.pow(2) + y.pow(2) + z.pow(2)
                                    }).unwrap();

    println!("{:?}", slowest_particle.num);
}
