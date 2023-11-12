use std::{env, fs};

use anyhow::Context;
use day_11::{solve_part_1, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let p: Problem = content.parse()?;

    let occupied_seats = solve_part_1(p);
    println!("Part 1: {occupied_seats}");

    Ok(())
}
