use std::{env, fs};

use anyhow::Context;
use day_01::{find_three_sum, find_two_sum, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let Problem { mut entries } = content.parse()?;
    entries.sort_unstable();

    let target_sum = 2020;

    let (a, b) = find_two_sum(&entries, target_sum).unwrap();
    println!("Part 1: {} * {} = {}", a, b, a * b);

    let (a, b, c) = find_three_sum(&entries, target_sum).unwrap();
    println!("Part 2: {} * {} * {} = {}", a, b, c, a * b * c);

    Ok(())
}
