use std::{env, fs};

use anyhow::Context;
use day_12::{solve_part_1, solve_part_2, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let p: Problem = content.parse()?;

    let manhattan_distance = solve_part_1(&p)?;
    println!("Part 1: {manhattan_distance}");

    let manhattan_distance = solve_part_2(&p)?;
    println!("Part 2: {manhattan_distance}");

    Ok(())
}
