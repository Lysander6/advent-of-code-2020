use std::{env, fs};

use anyhow::Context;
use day_01::{find_three_sum_naive, find_two_sum_naive, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let Problem { entries } = content.parse()?;
    let target_sum = 2020;

    let (a, b) = find_two_sum_naive(&entries, target_sum).unwrap();
    println!("Part 1: {} * {} = {}", a, b, a * b);

    let (a, b, c) = find_three_sum_naive(&entries, target_sum).unwrap();
    println!("Part 2: {} * {} * {} = {}", a, b, c, a * b * c);

    Ok(())
}
