use std::{env, fs};

use anyhow::Context;
use day_04::{count_passports_with_required_fields, Problem, count_valid_passports};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let p: Problem = content.parse()?;

    let passports_with_required_fields_count = count_passports_with_required_fields(&p);
    println!("Part 1: {passports_with_required_fields_count}");

    let valid_passports_count = count_valid_passports(&p);
    println!("Part 2: {valid_passports_count}");

    Ok(())
}
