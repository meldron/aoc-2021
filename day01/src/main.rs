use anyhow::{anyhow, Result};
use std::fs;

static INPUT_PATH: &str = "input.txt";

fn load_input(path: &str) -> Result<Vec<u16>> {
    let raw = fs::read_to_string(path)?;

    raw.lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u16>())
        .map(|r| r.map_err(|e| anyhow!(e)))
        .collect()
}

fn count_increases(values: &[u16]) -> usize {
    values.windows(2).filter(|v| v[0] < v[1]).count()
}

fn three_measurements(values: &[u16]) -> usize {
    let three_measurement_windows: Vec<u16> =
        values.windows(3).map(|w| w.into_iter().sum()).collect();

    count_increases(&three_measurement_windows)
}

fn main() -> Result<()> {
    let input = load_input(INPUT_PATH)?;

    println!("Part 1: {}", count_increases(&input));
    println!("Part 1: {}", three_measurements(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
