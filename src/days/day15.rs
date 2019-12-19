use crate::common::Point;
use crate::intcode::Computer;
use anyhow::{anyhow, Result};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Copy, Clone)]
enum Location {
    Wall,
    Traversable(TraversableLocation),
}

#[derive(Debug, Copy, Clone)]
enum TraversableLocation {
    Empty,
    Oxygen,
}

#[derive(Debug, Clone)]
struct Maze {
    queue: VecDeque<(Computer, TraversableLocation, Point, u64)>,
    visited: HashMap<Point, u64>,
}

impl Maze {
    pub fn new(source: &str) -> Result<Maze> {
        Self::new_from_computer(Computer::new_from_str(source)?)
    }

    pub fn new_from_computer(computer: Computer) -> Result<Maze> {
        let start = Point { x: 0, y: 0 };

        let mut queue = VecDeque::new();
        queue.push_back((computer, TraversableLocation::Empty, start, 0));

        Ok(Maze {
            queue,
            visited: HashMap::new(),
        })
    }
}

impl Iterator for Maze {
    type Item = (Computer, TraversableLocation, Point, u64);

    fn next(&mut self) -> Option<(Computer, TraversableLocation, Point, u64)> {
        if self.queue.is_empty() {
            return None;
        }

        let (current_computer, traversable_location, point, distance) =
            self.queue.pop_front().unwrap();

        for direction in vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            let direction_input = get_direction_input(direction);
            let direction_point = get_direction_point(direction);

            if self.visited.contains_key(&point) {
                continue;
            }

            let mut next_computer = current_computer.clone();
            let (_, output) = next_computer.run_until_blocked(direction_input).unwrap();

            let location = match output[0] {
                0 => Location::Wall,
                1 => Location::Traversable(TraversableLocation::Empty),
                2 => Location::Traversable(TraversableLocation::Oxygen),
                _ => unreachable!(),
            };

            match location {
                Location::Traversable(tl) => {
                    self.queue.push_back((
                        next_computer,
                        tl,
                        point + direction_point,
                        distance + 1,
                    ));
                }
                _ => {}
            }
        }

        self.visited.insert(point, distance);
        Some((current_computer, traversable_location, point, distance))
    }
}

pub fn part1(source: &str) -> Result<String> {
    let maze = Maze::new(source)?;
    for (_, location, _, distance) in maze {
        match location {
            TraversableLocation::Oxygen => return Ok(distance.to_string()),
            _ => {}
        }
    }

    Err(anyhow!("Traversed map without finding a solution"))
}

pub fn part2(source: &str) -> Result<String> {
    let (computer, _, _, _) = Maze::new(source)?
        .find(|(_, location, _, _)| match location {
            TraversableLocation::Oxygen => true,
            _ => false,
        })
        .unwrap();

    let max_dist = Maze::new_from_computer(computer)?
        .map(|(_, _, _, distance)| distance)
        .max_by_key(|x| *x)
        .unwrap();

    Ok((max_dist - 1).to_string())
}

fn get_direction_point(direction: Direction) -> Point {
    match direction {
        Direction::North => Point { x: 0, y: 1 },
        Direction::South => Point { x: 0, y: -1 },
        Direction::West => Point { x: -1, y: 0 },
        Direction::East => Point { x: 1, y: 0 },
    }
}

fn get_direction_input(direction: Direction) -> Vec<i64> {
    vec![match direction {
        Direction::North => 1,
        Direction::South => 2,
        Direction::West => 3,
        Direction::East => 4,
    }]
}
