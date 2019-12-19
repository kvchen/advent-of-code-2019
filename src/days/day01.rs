use anyhow::Result;

pub fn part1(input_string: &str) -> Result<String> {
    Ok(get_fuel_requirement(input_string, |mass| mass / 3 - 2))
}

pub fn part2(input_string: &str) -> Result<String> {
    fn mass_to_fuel(mass: u64) -> u64 {
        match (mass / 3).checked_sub(2) {
            Some(fuel) => fuel + mass_to_fuel(fuel),
            None => 0,
        }
    }

    Ok(get_fuel_requirement(input_string, mass_to_fuel))
}

fn get_fuel_requirement(input_string: &str, mass_to_fuel: fn(u64) -> u64) -> String {
    input_string
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(mass_to_fuel)
        .sum::<u64>()
        .to_string()
}
