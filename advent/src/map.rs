use std::fmt;

use crate::grid;
use crate::grid::Coord;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rotation {
    Left,
    Right,
}

/// A location on a grid map which contains the cardinal directions.
pub struct Location {
    starting_location: Coord,
    location: Coord,
}

/// The point-of-view on a grid map which contains the cardinal directions.
pub struct Viewpoint {
    starting_direction: Direction,
    direction: Direction,
}

impl Direction {
    pub fn from(c: char) -> Self {
        match c {
            'N' => Direction::North,
            'E' => Direction::East,
            'S' => Direction::South,
            'W' => Direction::West,
            _ => panic!("Unhandled direction character: {}", c),
        }
    }
}

impl Rotation {
    pub fn from(c: char) -> Self {
        match c {
            'L' => Rotation::Left,
            'R' => Rotation::Right,
            _ => panic!("Unhandled rotation character: {}", c),
        }
    }
}

impl Location {
    /// Create a new location starting at (0,0).
    pub fn new() -> Self {
        let starting_location = (0, 0);
        Location {
            starting_location,
            location: starting_location,
        }
    }

    /// Create a new location starting after applying the given movements
    pub fn new_at_location(movements: &[(Direction, i64)]) -> Self {
        let mut location = Location::new();
        movements.iter().for_each(|&(direction, value)| location.movement(direction, value));
        location
    }

    /// Get the location in a east/west and north/south tuple.
    pub fn location(&self) -> ((Direction, i64), (Direction, i64)) {
        let horizontal = if self.location.0 >= 0 {
            (Direction::East, self.location.0)
        } else {
            (Direction::West, self.location.0.abs())
        };
        let vertical = if self.location.1 <= 0 {
            (Direction::North, self.location.1.abs())
        } else {
            (Direction::South, self.location.1)
        };
        (horizontal, vertical)
    }

    pub fn move_to_location(&mut self, location: &Location) {
        self.location = ((self.location.0 + location.location.0), (self.location.1 + location.location.1));
    }

    /// Move the location in the given direction with the given value.
    pub fn movement(&mut self, direction: Direction, value: i64) {
        let current_location = self.location;
        let next_location = match direction {
            Direction::North => grid::Direction::Up.next_coord_by_value(current_location, value),
            Direction::East => grid::Direction::Right.next_coord_by_value(current_location, value),
            Direction::South => grid::Direction::Down.next_coord_by_value(current_location, value),
            Direction::West => grid::Direction::Left.next_coord_by_value(current_location, value),
        };
        self.location = next_location;
    }

    /// Rotate this location assuming its relative to (0, 0) 
    pub fn relative_rotate(&mut self, rotation: Rotation, value: i64) {
        let ((horizontal_dir, horizontal_val), (vertical_dir, vertical_val)) = self.location();
        let new_horizontal_dir = Viewpoint::rotate_direction(horizontal_dir, rotation, value);
        let new_vertical_dir = Viewpoint::rotate_direction(vertical_dir, rotation, value);
        let new_location = match new_horizontal_dir {
            Direction::North|Direction::South => (Location::coordinate_value(new_vertical_dir, vertical_val), Location::coordinate_value(new_horizontal_dir, horizontal_val)),
            Direction::East|Direction::West => (Location::coordinate_value(new_horizontal_dir, horizontal_val), Location::coordinate_value(new_vertical_dir, vertical_val)),
        };
        self.location = new_location;
    }

    /// Return the manhattan distance of the location from the starting position.
    pub fn manhattan_distance(&self) -> i64 {
        (self.location.0 - self.starting_location.0).abs() + (self.location.1 - self.starting_location.1).abs()
    }

    /// Return the coordinate value associated with the given direction.
    fn coordinate_value(direction: Direction, value: i64) -> i64 {
        let abs = value.abs();
        match direction {
            Direction::North => -1 * abs,
            Direction::East => abs,
            Direction::South => abs,
            Direction::West => -1 * abs,
        }
    }
}

impl Viewpoint {
    /// Create a new location usin gthe starting direction.
    pub fn new(starting_direction: Direction) -> Self {
        Viewpoint {
            starting_direction,
            direction: starting_direction,
        }
    }

    /// Return the current facing direction.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Rotate the point-of-view in the given direction with the given value.
    pub fn rotate(&mut self, rotation: Rotation, value: i64) {
        self.direction = Viewpoint::rotate_direction(self.direction, rotation, value);
    }

    /// Rotate the given direction, returning the resulting direction.
    pub fn rotate_direction(direction: Direction, rotation: Rotation, value: i64) -> Direction {
        match (direction, rotation, value) {
            (Direction::North, Rotation::Left, 90)  => Direction::West,
            (Direction::North, Rotation::Left, 180) => Direction::South,
            (Direction::North, Rotation::Left, 270) => Direction::East,
            (Direction::East,  Rotation::Left, 90)  => Direction::North,
            (Direction::East,  Rotation::Left, 180) => Direction::West,
            (Direction::East,  Rotation::Left, 270) => Direction::South,
            (Direction::South, Rotation::Left, 90)  => Direction::East,
            (Direction::South, Rotation::Left, 180) => Direction::North,
            (Direction::South, Rotation::Left, 270) => Direction::West,
            (Direction::West,  Rotation::Left, 90)  => Direction::South,
            (Direction::West,  Rotation::Left, 180) => Direction::East,
            (Direction::West,  Rotation::Left, 270) => Direction::North,
            (_, Rotation::Right, v) => Viewpoint::rotate_direction(direction, Rotation::Left, 360 - v),
            _ => panic!("Unhandled tuple: {} {} {}", direction, rotation, value),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Direction::North => 'N',
            Direction::East => 'E',
            Direction::South => 'S',
            Direction::West => 'W',
        };
        write!(f, "{}", c)
    }
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Rotation::Left => 'L',
            Rotation::Right => 'R',
        };
        write!(f, "{}", c)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ((horizontal_dir, horizontal_val), (vertical_dir, vertical_val)) = self.location();
        write!(f, "({} {}, {} {})", horizontal_dir, horizontal_val, vertical_dir, vertical_val)
    }
}