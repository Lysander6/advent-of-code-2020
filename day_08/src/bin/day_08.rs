use std::{env, fs};

use anyhow::Context;
use day_08::{run_until_first_loop, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let p: Problem = content.parse()?;

    let (accumulator_value, _) = run_until_first_loop(&p)?;
    println!("Part 1: {accumulator_value}");

    Ok(())
}
