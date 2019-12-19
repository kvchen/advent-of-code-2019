use anyhow::{anyhow, Result};
use std::io::Read;
use structopt::StructOpt;

mod common;
mod days;
mod intcode;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt()]
    question: u32,

    #[structopt()]
    part: u32,
}

struct Parts(fn(&str) -> Result<String>, fn(&str) -> Result<String>);

fn main() -> Result<()> {
    let args = Opt::from_args();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let question = args.question;
    let part = args.part;

    let parts = match question {
        1 => Ok(Parts(days::day01::part1, days::day01::part2)),
        2 => Ok(Parts(days::day02::part1, days::day02::part2)),
        5 => Ok(Parts(days::day05::part1, days::day05::part2)),
        9 => Ok(Parts(days::day09::part1, days::day09::part2)),
        11 => Ok(Parts(days::day11::part1, days::day11::part2)),
        15 => Ok(Parts(days::day15::part1, days::day15::part2)),
        _ => Err(anyhow!("Question {} not implemented", question)),
    }?;

    let result = match part {
        1 => parts.0(&input),
        2 => parts.1(&input),
        _ => Err(anyhow!("Part {} not implemented", part)),
    }?;

    println!("{}", result);
    Ok(())
}
