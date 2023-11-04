use std::{env, fs};

use anyhow::Context;
use day_06::{sum_yes_answers, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let p: Problem = content.parse()?;

    let yes_answers = sum_yes_answers(&p);
    println!("Part 1: {yes_answers}");

    Ok(())
}
