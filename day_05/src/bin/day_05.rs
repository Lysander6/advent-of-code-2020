use std::{env, fs};

use anyhow::{anyhow, Context};
use day_05::{find_max_seat_id, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let p: Problem = content.parse()?;

    let max_seat_id =
        find_max_seat_id(&p).ok_or_else(|| anyhow!("couldn't find max seat id (empty list?)"))?;
    println!("Part 1: {max_seat_id}");

    Ok(())
}
