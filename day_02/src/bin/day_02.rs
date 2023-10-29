use std::{env, fs};

use anyhow::Context;
use day_02::{count_valid_passwords, validate_password, validate_password_new_policy, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let Problem { passwords } = content.parse()?;

    let count = count_valid_passwords(&passwords, validate_password);
    println!("Part 1: {count}");

    let count_new_policy = count_valid_passwords(&passwords, validate_password_new_policy);
    println!("Part 2: {count_new_policy}");

    Ok(())
}
