use std::{env, fs};

use anyhow::Context;
use day_03::{count_collisions, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let p: Problem = content.parse()?;

    let count = count_collisions(&p, 3, 1);
    println!("Part 1: {count}");

    let result: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .map(|(right, down)| count_collisions(&p, right, down))
        .iter()
        .product();
    println!("Part 2: {result}");

    Ok(())
}
