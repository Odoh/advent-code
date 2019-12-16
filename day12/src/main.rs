use advent::InputSnake;
use itertools::Itertools;

type Coord = (i64, i64, i64);

#[derive(Debug, Clone, PartialEq)]
struct Moon {
    position: Coord,
    velocity: Coord,
}

impl Moon {
    pub fn new(position: Coord) -> Self {
        let velocity = (0, 0, 0);
        Moon {
            position,
            velocity,
        }
    }

    pub fn energy(&self) -> i64 {
        let (px, py, pz) = self.position;
        let (vx, vy, vz) = self.velocity;
        let pe = px.abs() + py.abs() + pz.abs();
        let ke = vx.abs() + vy.abs() + vz.abs();
        pe * ke
    }

    pub fn apply_velocity(&mut self) {
        let (px, py, pz) = self.position;
        let (vx, vy, vz) = self.velocity;
        self.position = (px + vx, py + vy, pz + vz)
    }

    pub fn apply_gravity(&mut self, other_moon: &Moon) {
        let (px, py, pz) = self.position;
        let (vx, vy, vz) = self.velocity;
        let (opx, opy, opz) = other_moon.position;
        let vx = vx + Moon::velocity_delta_from_gravity(px, opx);
        let vy = vy + Moon::velocity_delta_from_gravity(py, opy);
        let vz = vz + Moon::velocity_delta_from_gravity(pz, opz);
        self.velocity = (vx, vy, vz)
    }

    fn velocity_delta_from_gravity(p: i64, op: i64) -> i64 {
        if p > op {
            -1
        } else if p < op {
            1
        } else {
            0
        }
    }
}

fn parse_moon(s: &str) -> Moon {
    let position: Coord = s.split(',')
        .map(|s| s.trim().replace('<', "").replace('>', ""))
        .map(|s| s.split('=').nth(1).unwrap().parse::<i64>().expect("Integer position coordinates"))
        .tuples()
        .next().unwrap();
    Moon::new(position)
}

fn part_one() {
    let input = InputSnake::new("input");
    let mut moons = input.snake()
        .map(|s| parse_moon(&s))
        .collect::<Vec<Moon>>();

    for _ in 0..1000 {
        let moons_snapshot = moons.clone();

        // apply gravity for all moons
        moons.iter_mut()
            .for_each(|moon| moons_snapshot.iter()
                .for_each(|other_moon| moon.apply_gravity(other_moon)));

        // apply velocity for all moons
        moons.iter_mut().for_each(|moon| moon.apply_velocity());
    }
    let total_energy = moons.iter()
        .map(|moon| moon.energy())
        .sum::<i64>();

    println!("Part One: {:?}", total_energy);
}

fn part_two() {
    let input = InputSnake::new("input");
    let initial_moons = input.snake()
        .map(|s| parse_moon(&s))
        .collect::<Vec<Moon>>();

    let mut moons = initial_moons.clone();
    let mut i = 0;
    loop {
        let moons_snapshot = moons.clone();

        // apply gravity for all moons
        moons.iter_mut()
            .for_each(|moon| moons_snapshot.iter()
                .for_each(|other_moon| moon.apply_gravity(other_moon)));

        // apply velocity for all moons
        moons.iter_mut().for_each(|moon| moon.apply_velocity());

        if initial_moons == moons {
            break;
        }
        i += 1;
    }

    println!("Part Two: {:?}", i);
}

fn main() {
    part_one();
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_moon() {
        let s = "<x=1, y=3, z=-11>";
        let expected_position = (1, 3, -11);
        assert_eq!(expected_position, parse_moon(s).position);
    }
}
