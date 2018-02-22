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

fn update(particle: &mut Particle) {
    let (ax, ay, az) = particle.acc;
    let (vx, vy, vz) = particle.vel;
    let (px, py, pz) = particle.pos;

    // update the velocity and position
    let (nvx, nvy, nvz) = (vx + ax, vy + ay, vz + az);
    let (npx, npy, npz) = (px + nvx, py + nvy, pz + nvz);

    particle.pos = (npx, npy, npz);
    particle.vel = (nvx, nvy, nvz);
    particle.acc = (ax, ay, az);
}

fn main() {
    // let input = "example";
    // let input = "example_2";
    let input = "question";
    let mut particles = from_file(input);

    for i in 0..usize::max_value() {
        // find particles with the same position and remove them
        let remove_nums = particles.iter()
                                   .filter(|&particle| particles.iter().any(|p| particle.pos == p.pos &&
                                                                                particle.num != p.num))
                                   .map(|particle| particle.num)
                                   .collect::<Vec<usize>>();
        particles.retain(|particle| !remove_nums.iter().any(|&n| particle.num == n));

        if i % 10000 == 0 {
            println!("{} {}", i, particles.len());
        }

        // update the position of the particles
        for mut particle in particles.iter_mut() {
            update(&mut particle);
        }
    }
}
