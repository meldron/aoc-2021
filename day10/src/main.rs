use anyhow::{anyhow, bail, Error, Result};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BracketMeaning {
    Opened(Bracket),
    Closed(Bracket),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bracket {
    Parentheses,
    Square,
    Curly,
    Angle,
}

impl Bracket {
    pub fn error_score(&self) -> usize {
        match self {
            Bracket::Parentheses => 3,
            Bracket::Square => 57,
            Bracket::Curly => 1197,
            Bracket::Angle => 25137,
        }
    }

    pub fn complete_score(&self) -> usize {
        match self {
            Bracket::Parentheses => 1,
            Bracket::Square => 2,
            Bracket::Curly => 3,
            Bracket::Angle => 4,
        }
    }
}

impl TryFrom<char> for BracketMeaning {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        let bracket = match c {
            '(' => BracketMeaning::Opened(Bracket::Parentheses),
            ')' => BracketMeaning::Closed(Bracket::Parentheses),
            '[' => BracketMeaning::Opened(Bracket::Square),
            ']' => BracketMeaning::Closed(Bracket::Square),
            '<' => BracketMeaning::Opened(Bracket::Angle),
            '>' => BracketMeaning::Closed(Bracket::Angle),
            '{' => BracketMeaning::Opened(Bracket::Curly),
            '}' => BracketMeaning::Closed(Bracket::Curly),
            _ => bail!("unknown char"),
        };

        Ok(bracket)
    }
}

fn parse_line(line: String) -> Result<(usize, Vec<Bracket>)> {
    let mut bracket_list = Vec::<Bracket>::new();
    let mut error = 0;

    for c in line.chars() {
        let token = BracketMeaning::try_from(c)?;
        let last = bracket_list.last();

        match (last, token) {
            (None, BracketMeaning::Closed(b)) => error = b.error_score(),
            (_, BracketMeaning::Opened(b)) => bracket_list.push(b),
            (Some(last), BracketMeaning::Closed(b)) => {
                if b == *last {
                    bracket_list.pop();
                } else {
                    error = b.error_score();
                }
            }
        }

        if error != 0 {
            break;
        }
    }

    Ok((error, bracket_list))
}

fn complete_line_score(line: Vec<Bracket>) -> usize {
    line.into_iter()
        .rev()
        .fold(0, |acc, b| acc * 5 + b.complete_score())
}

pub trait Median: Iterator {
    /// Calculate median
    fn median(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord + Clone,
    {
        let v: Vec<_> = self.into_iter().sorted().collect();
        let median_index = v.len() / 2;

        v.get(median_index).cloned()
    }
}

impl<I: Iterator> Median for I {}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|s| s.trim().to_owned())
        .collect();

    let parsed_input = input
        .into_iter()
        .map(parse_line)
        .collect::<Result<Vec<_>>>()?;

    let total_error_score: usize = parsed_input.iter().map(|(e, _)| e).sum();
    println!("Total Error Score: {}", total_error_score);

    let incomplete_input: Vec<_> = parsed_input
        .into_iter()
        .filter(|(e, _)| *e == 0)
        .map(|(_, l)| l)
        .collect();

    let middle_completion_score = incomplete_input
        .into_iter()
        .map(complete_line_score)
        .median()
        .ok_or(anyhow!("Middle Completion Score not found"))?;

    println!("Middle Completion Score: {:?}", middle_completion_score);

    Ok(())
}
