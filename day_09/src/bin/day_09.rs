use std::{env, fs};

use anyhow::Context;
use day_09::{find_continuous_set_summing_to_value, find_first_not_following_the_rule, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let Problem { numbers } = content.parse()?;

    let value = find_first_not_following_the_rule(25, &numbers);
    println!("Part 1: {value}");

    let v = find_continuous_set_summing_to_value(value, &numbers);
    let min = v.iter().min().unwrap();
    let max = v.iter().max().unwrap();
    println!("Part 2: {}", min + max);

    Ok(())
}
