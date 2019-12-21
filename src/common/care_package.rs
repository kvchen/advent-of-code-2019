use crate::common::Point;
use crate::intcode::{Computer, IndexedParameter};
use anyhow::Result;
use itertools::join;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Neutral,
    Left,
    Right,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => " ",
                Tile::Wall => "█",
                Tile::Block => "░",
                Tile::Paddle => "▄",
                Tile::Ball => "■",
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Screen {
    data: Vec<Vec<Tile>>,
    pub score: u64,

    // Cached data from initial traversal
    pub paddle: Point,
    pub ball: Point,
    pub remaining_blocks: usize,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            data: vec![vec![Tile::Empty; 38]; 22],
            score: 0,

            paddle: Point { x: 0, y: 0 },
            ball: Point { x: 0, y: 0 },
            remaining_blocks: 0,
        }
    }

    pub fn update(&mut self, output: Vec<i64>) {
        for chunk in output.chunks(3) {
            if chunk[0] == -1 && chunk[1] == 0 {
                self.score = chunk[2] as u64;
                continue;
            }

            let point = Point {
                x: chunk[0],
                y: chunk[1],
            };

            let current_tile = self.data[point.y as usize][point.x as usize];
            let next_tile = match chunk[2] {
                0 => Tile::Empty,
                1 => Tile::Wall,
                2 => Tile::Block,
                3 => Tile::Paddle,
                4 => Tile::Ball,
                _ => unreachable!(),
            };

            if current_tile == Tile::Block && next_tile == Tile::Empty {
                self.remaining_blocks -= 1;
            } else if current_tile != Tile::Block && next_tile == Tile::Block {
                self.remaining_blocks += 1;
            } else if next_tile == Tile::Paddle {
                self.paddle = point;
            } else if next_tile == Tile::Ball {
                self.ball = point;
            }

            self.data[point.y as usize][point.x as usize] = next_tile;
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            join(self.data.iter().map(|row| join(row, "")), "\n")
        )
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    computer: Computer,
    pub screen: Screen,
}

impl Game {
    pub fn new(initial_memory: &str, has_credits: bool) -> Result<Self> {
        let mut computer = Computer::new_from_str(initial_memory)?;
        if has_credits {
            computer.set_value(IndexedParameter::Positional(0), 2);
        }

        let (_, initial_output) = computer.run_until_stopped(vec![])?;

        let mut screen = Screen::new();
        screen.update(initial_output);

        Ok(Self { computer, screen })
    }

    pub fn do_move(&mut self, direction: Direction) -> Result<&Screen> {
        let input = vec![match direction {
            Direction::Neutral => 0,
            Direction::Left => -1,
            Direction::Right => 1,
        }];

        let (_, output) = self.computer.run_until_stopped(input)?;
        self.screen.update(output);

        Ok(&self.screen)
    }
}
