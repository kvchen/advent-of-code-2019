use crate::common::care_package::{Direction, Game};
use anyhow::Result;

pub fn part1(source: &str) -> Result<String> {
    let mut game = Game::new(source, false)?;
    let screen = game.do_move(Direction::Neutral)?;

    Ok(screen.remaining_blocks.to_string())
}

pub fn part2(source: &str) -> Result<String> {
    let mut game = Game::new(source, true)?;
    let mut direction = Direction::Neutral;

    loop {
        let screen = game.do_move(direction)?;

        // print!("{}[2J", 27 as char);
        // println!("{}", screen);

        if screen.remaining_blocks == 0 {
            return Ok(screen.score.to_string());
        }

        direction = if screen.paddle.x < screen.ball.x {
            Direction::Right
        } else if screen.paddle.x > screen.ball.x {
            Direction::Left
        } else {
            Direction::Neutral
        };
    }
}
