use crate::intcode::{Computer, IndexedParameter};
use anyhow::{anyhow, Result};
use itertools::iproduct;

pub fn part1(source: &str) -> Result<String> {
    let mut computer = Computer::new_from_str(source)?;
    computer.set_value(IndexedParameter::Positional(1), 12);
    computer.set_value(IndexedParameter::Positional(2), 2);

    computer.run(vec![])?;
    Ok(computer.get_memory_value(0).to_string())
}

pub fn part2(source: &str) -> Result<String> {
    let target = 19690720;

    for (noun, verb) in iproduct!(0..100, 0..100) {
        let mut computer = Computer::new_from_str(source)?;
        computer.set_value(IndexedParameter::Positional(1), noun);
        computer.set_value(IndexedParameter::Positional(2), verb);

        computer.run(vec![])?;

        if computer.get_memory_value(0) == target {
            return Ok((100 * noun + verb).to_string());
        }
    }

    Err(anyhow!("Unable to find valid noun/verb combination"))
}
