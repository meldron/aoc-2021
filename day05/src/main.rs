use anyhow::{anyhow, Error, Result};
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let split: Vec<&str> = s.split(",").collect();

        let x_raw = *split.get(0).ok_or(anyhow!("x not found"))?;
        let y_raw = *split.get(1).ok_or(anyhow!("y not found"))?;

        let x: i32 = x_raw.trim().parse()?;
        let y: i32 = y_raw.trim().parse()?;

        Ok(Point { x, y })
    }
}

pub trait Direction {
    fn to_direction(&self) -> i32;
}

impl Direction for Ordering {
    fn to_direction(&self) -> i32 {
        match self {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

impl Point {
    pub fn directions(&self, other: &Point) -> (i32, i32) {
        let x_dir = other.x.cmp(&self.x).to_direction();
        let y_dir = other.y.cmp(&self.y).to_direction();

        (x_dir, y_dir)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn points(&self) -> LineIntoIterator {
        self.into_iter()
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let split: Vec<&str> = s.split(" -> ").collect();

        let start_raw = *split.get(0).ok_or(anyhow!("start not found"))?;
        let end_raw = *split.get(1).ok_or(anyhow!("end not found"))?;

        let start = Point::from_str(start_raw.trim())?;
        let end = Point::from_str(end_raw.trim())?;

        Ok(Line { start, end })
    }
}

impl IntoIterator for Line {
    type Item = Point;

    type IntoIter = LineIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        let (x_dir, y_dir) = self.start.directions(&self.end);

        LineIntoIterator {
            start: self.start,
            current: None,
            end: self.end,
            x_dir,
            y_dir,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LineIntoIterator {
    pub start: Point,
    pub end: Point,

    pub current: Option<Point>,

    pub x_dir: i32,
    pub y_dir: i32,
}

impl Iterator for LineIntoIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            self.current = Some(self.start);
            return Some(self.start);
        }

        let current = self.current.unwrap();

        if current.x == self.end.x && current.y == self.end.y {
            return None;
        }

        self.current = Some(Point {
            x: current.x + self.x_dir,
            y: current.y + self.y_dir,
        });

        self.current
    }
}

fn load_lines(path: &str) -> Result<Vec<Line>> {
    std::fs::read_to_string(path)?
        .lines()
        .map(|l| Line::from_str(l))
        .collect()
}

fn draw_lines(lines: &[Line]) -> HashMap<Point, usize> {
    lines.iter().fold(HashMap::new(), |mut map, line| {
        line.points().for_each(|p| *map.entry(p).or_insert(0) += 1);

        map
    })
}

fn num_overlaps(diagram: &HashMap<Point, usize>) -> usize {
    diagram.values().filter(|v| **v > 1).count()
}

fn main() -> Result<()> {
    let lines = load_lines("input.txt")?;

    let lines_p1: Vec<Line> = lines
        .clone()
        .into_iter()
        .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
        .collect();

    let diagram_p1 = draw_lines(&lines_p1);
    let p1 = num_overlaps(&diagram_p1);
    println!("Part1: {}", p1);

    let diagram_p2 = draw_lines(&lines);
    let p2 = num_overlaps(&diagram_p2);
    println!("Part2: {} (points: {})", p2, diagram_p2.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_parsing() {
        let line = Line::from_str("60,28 -> 893,861");
        let expected = Line {
            start: Point { x: 60, y: 28 },
            end: Point { x: 893, y: 861 },
        };

        assert_eq!(line.unwrap(), expected);
    }

    #[test]
    fn get_point() {
        let line = Line::from_str("9,7 -> 7,7").expect("line parsing failed");
        let points: Vec<Point> = line.points().collect();

        let expected = vec![
            Point { x: 9, y: 7 },
            Point { x: 8, y: 7 },
            Point { x: 7, y: 7 },
        ];
        assert_eq!(points, expected);
    }

    #[test]
    fn get_point_p2() {
        let line = Line::from_str("9,7 -> 7,9").expect("line parsing failed");
        let points: Vec<Point> = line.points().collect();

        let expected = vec![
            Point { x: 9, y: 7 },
            Point { x: 8, y: 8 },
            Point { x: 7, y: 9 },
        ];
        assert_eq!(points, expected);
    }
}
