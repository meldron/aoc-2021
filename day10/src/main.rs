use anyhow::{bail, Error, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    Opened(Bracket),
    Closed(Bracket),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bracket {
    Parentheses,
    Square,
    Angle,
    Curly,
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

impl TryFrom<char> for Token {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        let bracket = match c {
            '(' => Token::Opened(Bracket::Parentheses),
            ')' => Token::Closed(Bracket::Parentheses),
            '[' => Token::Opened(Bracket::Square),
            ']' => Token::Closed(Bracket::Square),
            '<' => Token::Opened(Bracket::Angle),
            '>' => Token::Closed(Bracket::Angle),
            '{' => Token::Opened(Bracket::Curly),
            '}' => Token::Closed(Bracket::Curly),
            _ => bail!("unknown char"),
        };

        Ok(bracket)
    }
}

fn parse_line(line: &str) -> Result<(usize, Vec<Token>)> {
    let mut token_list = Vec::<Token>::new();
    let mut error = 0;

    for c in line.trim().chars() {
        let token = Token::try_from(c)?;
        let last = token_list.last();

        match (last, token) {
            (None, Token::Closed(b)) => {
                error = b.error_score();
                break;
            }
            (_, Token::Opened(_)) => token_list.push(token),
            (Some(Token::Opened(last)), Token::Closed(b)) => {
                if b == *last {
                    token_list.pop();
                } else {
                    error = b.error_score();
                    break;
                }
            }
            (Some(Token::Closed(_)), Token::Closed(b)) => {
                error = b.error_score();
                break;
            }
        }
    }

    Ok((error, token_list))
}

fn complete_line(line: &mut Vec<Token>) -> Result<usize> {
    let mut score: usize = 0;

    while let Some(t) = line.pop() {
        match t {
            Token::Opened(b) => {
                score = score * 5 + b.complete_score();
            }
            Token::Closed(_) => bail!("closed token found"),
        }
    }

    Ok(score)
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|s| s.to_owned())
        .collect();

    let parsed_input: Vec<(usize, Vec<Token>)> = input
        .into_iter()
        .map(|l| parse_line(&l))
        .collect::<Result<Vec<_>>>()?;

    let total_error_score: usize = parsed_input.iter().map(|(e, _)| e).sum();
    println!("Total Error Score: {}", total_error_score);

    let mut incomplete_input: Vec<Vec<Token>> = parsed_input
        .into_iter()
        .filter(|(e, _)| *e == 0)
        .map(|(_, l)| l)
        .collect();

    let mut complete_scores: Vec<usize> = incomplete_input
        .iter_mut()
        .map(|l| complete_line(l))
        .collect::<Result<Vec<_>>>()?;

    complete_scores.sort();
    let middle_completion_score = complete_scores[complete_scores.len() / 2];

    println!("Middle Completion Score: {}", middle_completion_score);

    Ok(())
}
