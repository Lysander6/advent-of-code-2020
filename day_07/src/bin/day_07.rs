use std::{env, fs};

use anyhow::Context;
use day_07::{count_containing_bags, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let p: Problem = content.parse()?;

    let containing_bags_count = count_containing_bags(&p, "shiny gold")?;
    println!("Part 1: {containing_bags_count}");

    Ok(())
}
