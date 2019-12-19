use crate::common::{Direction, Point};
use crate::intcode::{Computer, StoppedResult};
use anyhow::Result;
use itertools::join;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
enum Color {
    Black,
    White,
}

#[derive(Debug, Clone)]
struct ShipHullExplorer {
    computer: Computer,
    position: Point,
    direction: Direction,
    white: HashSet<Point>,
    status: StoppedResult,
}

impl ShipHullExplorer {
    fn new(serialized_memory: &str, white: HashSet<Point>) -> Result<ShipHullExplorer> {
        Ok(ShipHullExplorer {
            computer: Computer::new_from_str(serialized_memory)?,
            position: Point { x: 0, y: 0 },
            direction: Direction::North,
            white,
            status: StoppedResult::Blocked,
        })
    }
}

impl Iterator for ShipHullExplorer {
    type Item = (Point, Color);

    fn next(&mut self) -> Option<(Point, Color)> {
        if let StoppedResult::Halted = self.status {
            return None;
        }

        let input = vec![if self.white.contains(&self.position) {
            1
        } else {
            0
        }];

        let (status, output) = self.computer.run_until_stopped(input).unwrap();

        let color = match output[0] {
            0 => {
                self.white.remove(&self.position);
                Color::Black
            }
            1 => {
                self.white.insert(self.position);
                Color::White
            }
            _ => unreachable!(),
        };

        match output[1] {
            // Left 90 degrees
            0 => {
                self.direction = self.direction.rotate_left();
            }

            // Right 90 degrees
            1 => {
                self.direction = self.direction.rotate_right();
            }

            _ => unreachable!(),
        }

        let output = Some((self.position, color));

        self.status = status;
        self.position = self.position + self.direction.get_offset();

        output
    }
}

pub fn part1(source: &str) -> Result<String> {
    let explorer = ShipHullExplorer::new(source, HashSet::new())?;
    let mut painted: HashSet<Point> = HashSet::new();

    for (point, _) in explorer {
        painted.insert(point);
    }

    Ok(painted.len().to_string())
}

pub fn part2(source: &str) -> Result<String> {
    let mut white = HashSet::new();
    white.insert(Point { x: 0, y: 0 });

    let explorer = ShipHullExplorer::new(source, white)?;
    let mut white_points: HashSet<Point> = HashSet::new();

    for (point, color) in explorer {
        match color {
            Color::Black => {
                white_points.remove(&point);
            }
            Color::White => {
                white_points.insert(point);
            }
        }
    }

    let points = Vec::from_iter(white_points.clone());
    let points_slice = &points[..];

    let min_x = points_slice.into_iter().min_by_key(|p| p.x).unwrap().x;
    let min_y = points_slice.into_iter().min_by_key(|p| p.y).unwrap().y;
    let max_x = points_slice.into_iter().max_by_key(|p| p.x).unwrap().x;
    let max_y = points_slice.into_iter().max_by_key(|p| p.y).unwrap().y;

    let output = join(
        (min_y..max_y + 1).rev().map(|y| {
            join(
                (min_x..max_x + 1).map(|x| {
                    let point = Point { x, y };
                    if white_points.contains(&point) {
                        '#'
                    } else {
                        ' '
                    }
                }),
                "",
            )
        }),
        "\n",
    );

    Ok(output)
}
