use std::{collections::HashMap, fs::read_to_string};

use anyhow::{bail, Error, Result};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum Cucumber {
    East,
    South,
}

impl TryFrom<char> for Cucumber {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '>' => Ok(Self::East),
            'v' => Ok(Self::South),
            _ => bail!("unknown cucumber"),
        }
    }
}

type Pos = (usize, usize);
type Floor = HashMap<Pos, Cucumber>;

#[derive(Clone, Debug)]
struct SeaFloor {
    pub current: Floor,
    pub history: Vec<Floor>,
    pub width: usize,
    pub depth: usize,
}

impl SeaFloor {
    fn new(input: &str) -> Self {
        let history = vec![];
        let depth = input.lines().count();
        let width = input.lines().take(1).collect::<String>().len();

        let current = input
            .lines()
            .enumerate()
            .map(|(j, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .map(move |(i, c)| ((j, i), Cucumber::try_from(c)))
            })
            .flatten()
            .filter_map(|(pos, c)| match c {
                Ok(c) => Some((pos, c)),
                Err(_) => None,
            })
            .collect();

        Self {
            current,
            history,
            width,
            depth,
        }
    }

    fn next_step(&mut self) -> usize {
        let mut changed = 0;

        let mut update = |to_update: &mut Floor, from: &Floor, (pos, cucumber)| {
            let next_pos = self.next_pos(pos, cucumber);

            let overwritten = match from.get(&next_pos) {
                None => {
                    changed += 1;
                    to_update.insert(next_pos, *cucumber)
                }
                Some(_) => to_update.insert(*pos, *cucumber),
            };

            if overwritten.is_some() {
                panic!("value overwritten {:?} {:?}", pos, next_pos);
            }
        };

        // first half step we keep the south cucumbers
        let mut first_half = self
            .current
            .iter()
            .filter(|(_, cucumber)| matches!(cucumber, Cucumber::South))
            .map(|(pos, cucumber)| (*pos, *cucumber))
            .collect();

        // and update east cucumbers only
        self.current
            .iter()
            .filter(|(_, cucumber)| matches!(cucumber, Cucumber::East))
            .for_each(|d| update(&mut first_half, &self.current, d));

        // second half we keep the east cucumbers
        let mut second_half: Floor = first_half
            .iter()
            .filter(|(_, c)| matches!(c, Cucumber::East))
            .map(|(pos, cucumber)| (*pos, *cucumber))
            .collect();

        // and update south cucumbers only
        first_half
            .iter()
            .filter(|(_, cucumber)| matches!(cucumber, Cucumber::South))
            .for_each(|d| update(&mut second_half, &first_half, d));

        self.history.push(self.current.clone());
        self.current = second_half;

        changed
    }

    fn next_pos(&self, pos: &Pos, cucumber: &Cucumber) -> Pos {
        match cucumber {
            Cucumber::East => (pos.0, (pos.1 + 1) % self.width),
            Cucumber::South => ((pos.0 + 1) % self.depth, pos.1),
        }
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let mut sea_floor = SeaFloor::new(&input);

    let mut step: usize = 0;
    loop {
        let changes = sea_floor.next_step();

        step += 1;

        if changes == 0 {
            break;
        }
    }

    println!("step: {}", step);

    Ok(())
}
