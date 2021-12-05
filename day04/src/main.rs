use anyhow::{anyhow, bail, Context, Error, Result};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub type PosIdx = usize;
pub type RowPos = PosIdx;
pub type ColPos = PosIdx;

pub type Counter = HashMap<PosIdx, usize>;

pub type Pos = (RowPos, ColPos);

#[derive(Clone, Debug)]
pub struct BingoBoard {
    pub dim: usize,

    pub board: HashMap<u8, Pos>,

    pub row_counter: Counter,
    pub col_counter: Counter,

    pub marked: HashSet<u8>,
}

impl BingoBoard {
    pub fn create(fields: &Vec<Vec<u8>>) -> Result<Self> {
        let mut board = HashMap::new();
        let row_counter = HashMap::new();
        let col_counter = HashMap::new();
        let marked = HashSet::new();

        let dim = fields.len();

        fields.iter().enumerate().try_for_each(|(row_idx, row)| {
            if row.len() != dim {
                bail!("row {} has wrong length {} not {}", row_idx, row.len(), dim);
            }

            row.iter().enumerate().for_each(|(col_dix, value)| {
                board.insert(*value, (row_idx, col_dix));
            });

            Ok(())
        })?;

        Ok(BingoBoard {
            board,
            dim,
            col_counter,
            row_counter,
            marked,
        })
    }

    pub fn unmarked_sum(&self) -> usize {
        self.board
            .keys()
            .filter(|k| !self.marked.contains(k))
            .map(|k| *k as usize)
            .sum()
    }

    pub fn mark(&mut self, v: u8) -> bool {
        let pos = self.board.get(&v);

        if pos.is_none() {
            return false;
        }

        self.marked.insert(v);

        let (row_idx, col_idx) = pos.unwrap();

        let row_counter = self.row_counter.entry(*row_idx).or_insert(0);
        *row_counter += 1;

        let col_counter = self.col_counter.entry(*col_idx).or_insert(0);
        *col_counter += 1;

        if *row_counter == self.dim || *col_counter == self.dim {
            return true;
        }

        false
    }

    pub fn mark_all(&mut self, values: &[u8]) -> Option<(usize, usize)> {
        for (i, v) in values.iter().enumerate() {
            if self.mark(*v) {
                return Some((i, *v as usize * self.unmarked_sum()));
            }
        }

        None
    }
}

impl FromStr for BingoBoard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .lines()
            .map(|l| {
                l.split(" ")
                    .filter(|s| *s != "")
                    .map(|v| v.trim().parse::<u8>().map_err(|e| anyhow!(e)))
                    .collect::<Result<Vec<u8>>>()
            })
            .collect::<Result<Vec<Vec<u8>>>>()?;

        BingoBoard::create(&fields)
    }
}

fn load_input(path: &str) -> Result<(Vec<u8>, Vec<BingoBoard>)> {
    let raw = std::fs::read_to_string(path)?;

    let drawn_raw: String = raw.lines().take(1).collect();

    let drawn = drawn_raw
        .split(",")
        .map(|s| s.trim().parse::<u8>().map_err(|e| anyhow!(e)))
        .collect::<Result<Vec<u8>>>()
        .context("Parsing Drawn")?;

    let boards: Vec<BingoBoard> = raw
        .split("\n\n")
        .skip(1)
        .map(|s| BingoBoard::from_str(s))
        .collect::<Result<Vec<BingoBoard>>>()
        .context("Parsing Boards")?;

    Ok((drawn, boards))
}

fn main() -> Result<()> {
    let (drawn, boards) = load_input("input.txt")?;

    let boards_winner: Vec<(usize, usize)> = boards
        .into_iter()
        .map(|mut b| b.mark_all(&drawn))
        .filter_map(|b| b)
        .collect();

    let (first_values, first_score) = boards_winner
        .iter()
        .min_by(|(x_draw, _), (y_draw, _)| x_draw.cmp(y_draw))
        .ok_or(anyhow!("no first"))?;

    println!("Part1: {} ({})", first_score, first_values);

    let (last_values, last_score) = boards_winner
        .iter()
        .max_by(|(x_draw, _), (y_draw, _)| x_draw.cmp(y_draw))
        .ok_or(anyhow!("no last"))?;

    println!("Part2: {} ({})", last_score, last_values);

    Ok(())
}
