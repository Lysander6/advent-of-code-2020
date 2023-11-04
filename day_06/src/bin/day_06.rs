use std::{env, fs};

use anyhow::Context;
use day_06::{sum_shared_answers, sum_unique_answers, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let p: Problem = content.parse()?;

    let unique_answers = sum_unique_answers(&p);
    println!("Part 1: {unique_answers}");

    let shared_answers = sum_shared_answers(&p);
    println!("Part 2: {shared_answers}");

    Ok(())
}
