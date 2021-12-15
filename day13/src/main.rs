use anyhow::{anyhow, bail, Error, Result};
use std::{collections::HashSet, str::FromStr};

pub type Point = (usize, usize);
pub type Paper = HashSet<Point>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Left(usize),
    Up(usize),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, to_parse) = s
            .rsplit_once(" ")
            .ok_or(anyhow!("instruction first split not found"))?;
        let (dir, value_raw) = to_parse
            .split_once("=")
            .ok_or(anyhow!("instruction second split not found"))?;

        let value: usize = value_raw.trim().parse()?;

        match dir.trim() {
            "x" => Ok(Instruction::Left(value)),
            "y" => Ok(Instruction::Up(value)),
            _ => bail!("unknown instruction direction"),
        }
    }
}

fn load_paper(raw_lines: &str) -> Result<Paper> {
    let paper: Paper = raw_lines
        .lines()
        .map(|s| {
            let (x_raw, y_raw) = s
                .trim()
                .split_once(",")
                .ok_or(anyhow!("coord malformed: {}", s))?;

            let y: usize = y_raw.trim().parse()?;
            let x: usize = x_raw.trim().parse()?;

            Ok((y, x))
        })
        .collect::<Result<_>>()?;

    Ok(paper)
}

fn get_paper_dimension(paper: &Paper) -> Point {
    let y_max = paper
        .iter()
        .map(|p| p.0)
        .max()
        .ok_or(anyhow!("max y not found"))
        .unwrap();

    let x_max = paper
        .iter()
        .map(|p| p.1)
        .max()
        .ok_or(anyhow!("max x not found"))
        .unwrap();

    (y_max, x_max)
}

fn fold_paper(paper: &Paper, instruction: Instruction) -> Paper {
    paper
        .iter()
        .map(|(y, x)| match (y, x, instruction) {
            (y, x, Instruction::Left(v)) => {
                if *x < v {
                    (*y, *x)
                } else {
                    (*y, v * 2 - *x)
                }
            }
            (y, x, Instruction::Up(v)) => {
                if *y < v {
                    (*y, *x)
                } else {
                    (v * 2 - *y, *x)
                }
            }
        })
        .collect()
}

fn split_input(input: &str) -> Result<(Paper, Vec<Instruction>)> {
    let (paper_raw, instructions_raw) =
        input.split_once("\n\n").ok_or(anyhow!("input malformed"))?;

    let paper = load_paper(&paper_raw)?;
    let instructions = instructions_raw
        .lines()
        .map(|l| Instruction::from_str(l))
        .collect::<Result<_>>()?;

    Ok((paper, instructions))
}

fn print_paper(paper: &Paper) {
    let (y_max, x_max) = get_paper_dimension(paper);

    for y in 0..=y_max {
        for x in 0..=x_max {
            match paper.get(&(y, x)) {
                Some(_) => print!("#"),
                None => print!(" "),
            }
        }
        print!("\n");
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (paper_start, instructions) = split_input(&input)?;

    let folded_once = fold_paper(&paper_start, instructions[0]);

    println!("Part 1 | num points: {}", folded_once.len());

    let final_paper = instructions
        .into_iter()
        .fold(paper_start, |paper, instruction| {
            fold_paper(&paper, instruction)
        });

    println!("Part 2 | Code:");
    print_paper(&final_paper);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_left_instruction_correctly() {
        let instruction_raw = "fold along x=655";

        let instruction = Instruction::from_str(instruction_raw).unwrap();

        assert_eq!(instruction, Instruction::Left(655))
    }

    #[test]
    fn parse_up_instruction_correctly() {
        let instruction_raw = "fold along y=111";

        let instruction = Instruction::from_str(instruction_raw).unwrap();

        assert_eq!(instruction, Instruction::Up(111))
    }

    static SAMPLE_PAPER: &str = r"6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0";

    #[test]
    fn load_paper_correctly() {
        let paper_set = load_paper(SAMPLE_PAPER).unwrap();
        assert_eq!(paper_set.len(), 18);
    }

    #[test]
    fn fold_paper_correctly() {
        let paper_org = load_paper(SAMPLE_PAPER).unwrap();
        let paper_folded_once = fold_paper(&paper_org, Instruction::Up(7));
        assert_eq!(paper_folded_once.len(), 7);
    }
}
