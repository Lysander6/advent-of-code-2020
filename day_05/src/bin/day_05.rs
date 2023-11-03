use std::{env, fs};

use anyhow::{anyhow, Context};
use day_05::{find_my_seat_id, str_to_seat, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let Problem { boarding_passes } = content.parse()?;

    let seat_ids: Vec<_> = boarding_passes.iter().map(|s| str_to_seat(s)).collect();

    let max_seat_id = seat_ids
        .iter()
        .max()
        .ok_or_else(|| anyhow!("couldn't find max seat id (empty list?)"))?;
    println!("Part 1: {max_seat_id}");

    let my_seat_id = find_my_seat_id(&seat_ids)?;
    println!("Part 2: {my_seat_id}");

    Ok(())
}
