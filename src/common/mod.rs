use std::ops::{Add, Sub};

pub mod care_package;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn get_offset(&self) -> Point {
        match self {
            Direction::North => Point { x: 0, y: 1 },
            Direction::South => Point { x: 0, y: -1 },
            Direction::West => Point { x: -1, y: 0 },
            Direction::East => Point { x: 1, y: 0 },
        }
    }

    pub fn rotate_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }

    pub fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }
}
