use anyhow::{anyhow, Result};
use std::fs;
use std::str::FromStr;

static INPUT_PATH: &str = "input.txt";

#[derive(Clone, Copy, Debug, PartialEq)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();

        let command = *split.get(0).ok_or(anyhow!("Command not found"))?;

        let value_raw = *split.get(1).ok_or(anyhow!("Value not found"))?;
        let value = value_raw.parse::<i32>()?;

        match command {
            "forward" => Ok(Command::Forward(value)),
            "up" => Ok(Command::Up(value)),
            "down" => Ok(Command::Down(value)),
            _ => Err(anyhow!("Unknown Command")),
        }
    }
}

fn part_1(commands: &[Command]) -> i32 {
    let (depth, h_pos) = commands
        .iter()
        .fold((0, 0), |(mut depth, mut h_pos), command| {
            match command {
                Command::Forward(v) => h_pos += v,
                Command::Up(v) => depth -= v,
                Command::Down(v) => depth += v,
            };

            (depth, h_pos)
        });

    depth * h_pos
}

fn part_2(commands: &[Command]) -> i32 {
    let (depth, h_pos, _) =
        commands
            .iter()
            .fold((0, 0, 0), |(mut depth, mut h_pos, mut aim), command| {
                match command {
                    Command::Forward(v) => {
                        h_pos += v;
                        depth += aim * v;
                    }
                    Command::Up(v) => aim -= v,
                    Command::Down(v) => aim += v,
                };

                (depth, h_pos, aim)
            });

    depth * h_pos
}

fn load_input(path: &str) -> Result<Vec<Command>> {
    let raw = fs::read_to_string(path)?;

    raw.lines()
        .filter(|s| !s.is_empty())
        .map(|s| Command::from_str(s))
        .collect()
}

fn main() -> Result<()> {
    let commands = load_input(INPUT_PATH)?;

    let p1 = part_1(&commands);
    println!("Part 1: {}", p1);

    let p2 = part_2(&commands);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn command_parse_valid() {
        let expected = Command::Down(4);

        let command = Command::from_str("down 4").expect("error parsing command");

        assert_eq!(command, expected);
    }
}
