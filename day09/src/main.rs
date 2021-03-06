use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub type Point = (isize, isize);

pub type HightMap = HashMap<Point, u8>;

pub fn load_map(input: &str) -> Result<HightMap> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.trim().chars().enumerate().map(move |(j, c)| {
                let d = c
                    .to_digit(10)
                    .ok_or(anyhow!("invalid char '{}' in ({}, {})", c, i, j))?
                    as u8;
                Ok(((i as isize, j as isize), d))
            })
        })
        .flatten()
        .collect()
}

pub fn get_neighbors(map: &HightMap, point: Point) -> Vec<(Point, u8)> {
    let (i, j) = point;

    let up = (i + 1, j);
    let up_value = map.get(&up);

    let down = (i - 1, j);
    let down_value = map.get(&down);

    let right = (i, j + 1);
    let right_value = map.get(&right);

    let left = (i, j - 1);
    let left_value = map.get(&left);

    [
        (up, up_value.cloned()),
        (down, down_value.cloned()),
        (left, left_value.cloned()),
        (right, right_value.cloned()),
    ]
    .iter()
    .filter_map(|(point, o)| match o {
        Some(v) => Some((*point, *v)),
        None => None,
    })
    .collect()
}

pub fn find_low_points(map: &HightMap) -> Vec<(Point, u8)> {
    map.iter()
        .filter_map(|((i, j), v)| {
            let up = map.get(&(i + 1, *j));
            let down = map.get(&(i - 1, *j));
            let right = map.get(&(*i, j + 1));
            let left = map.get(&(*i, j - 1));

            let neighbors = [up, down, left, right];

            let num_neighbors = neighbors.iter().filter(|f| f.is_some()).count();

            let num_bigger = neighbors
                .iter()
                .filter_map(|o| o.as_deref())
                .filter(|o| *o > v)
                .count();

            if num_bigger == num_neighbors {
                Some(((*i, *j), *v))
            } else {
                None
            }
        })
        .collect()
}

pub fn calc_basin_sizes(map: &HightMap, low_points: Vec<Point>) -> Vec<usize> {
    let mut visited: HashSet<Point> = low_points.iter().map(|p| *p).collect();

    low_points
        .into_iter()
        .map(|low_point| {
            let mut to_visit = vec![low_point];
            let mut basin_size: usize = 0;

            while let Some(p) = to_visit.pop() {
                basin_size += 1;

                let neighbors = get_neighbors(map, p);
                neighbors.into_iter().for_each(|(neighbor, v)| {
                    if v < 9 && !visited.contains(&neighbor) {
                        to_visit.push(neighbor);
                        visited.insert(neighbor);
                    }
                });
            }

            basin_size
        })
        .collect()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let map = load_map(&input)?;
    let low_points_with_values = find_low_points(&map);

    let total_risk_level: usize = low_points_with_values
        .iter()
        .map(|(_, p)| (p + 1) as usize)
        .sum();

    let low_points: Vec<Point> = low_points_with_values
        .into_iter()
        .map(|(point, _)| point)
        .collect();

    let basin_sizes = calc_basin_sizes(&map, low_points);

    let three_largest_mult: usize = basin_sizes
        .into_iter()
        .sorted()
        .rev()
        .take(3)
        .fold(1, |acc, b| acc * b);

    println!("total_risk_level: {}", total_risk_level);
    println!("three_largest_mult: {}", three_largest_mult);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_map_working() {
        let map_raw = r"2199943210
        3987894921
        9856789892
        8767896789
        9899965678";

        let map = load_map(map_raw);

        assert_eq!(map.unwrap().len(), 50)
    }

    #[test]
    fn find_low_points_working() {
        let map_raw = r"2199943210
        3987894921
        9856789892
        8767896789
        9899965678";

        let map = load_map(map_raw).unwrap();
        let low_points_with_values = find_low_points(&map);

        assert_eq!(low_points_with_values.len(), 4)
    }
}
