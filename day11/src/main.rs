use anyhow::{anyhow, bail, Result};
use std::collections::{HashMap, HashSet};

pub type Point = (isize, isize);

pub type PosMap = HashMap<Point, usize>;

pub fn get_neighbors(map: &PosMap, point: Point) -> Vec<Point> {
    let (x, y) = point;

    let north = (1, 0);
    let north_east = (1, 1);
    let east = (0, 1);
    let south_east = (-1, 1);
    let south = (-1, 0);
    let south_west = (-1, -1);
    let west = (0, -1);
    let north_west = (1, -1);

    [
        north, north_east, east, south_east, south, south_west, west, north_west,
    ]
    .into_iter()
    .filter_map(|(i, j)| {
        let neighbor = (x + i, y + j);
        match map.get(&neighbor) {
            Some(_) => Some(neighbor),
            None => None,
        }
    })
    .collect()
}

pub fn load_map(input: &str) -> Result<PosMap> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.trim().chars().enumerate().map(move |(j, c)| {
                let d = c
                    .to_digit(10)
                    .ok_or(anyhow!("invalid char '{}' in ({}, {})", c, i, j))?
                    as usize;
                Ok(((i as isize, j as isize), d))
            })
        })
        .flatten()
        .collect()
}

fn next_step(last_step: &PosMap) -> (PosMap, usize) {
    let mut new_step: PosMap = last_step.iter().map(|(p, v)| (p.clone(), v + 1)).collect();

    let mut to_flash: Vec<Point> = new_step
        .iter()
        .filter(|(_, v)| **v == 10)
        .map(|(p, _)| p.clone())
        .collect();

    let mut flashed: HashSet<Point> = to_flash.iter().map(|p| *p).collect();

    while let Some(current) = to_flash.pop() {
        let neighbors = get_neighbors(&new_step, current);
        neighbors.into_iter().for_each(|neighbor_pos| {
            let neighbor_value = new_step.entry(neighbor_pos).or_default();

            *neighbor_value += 1;

            if *neighbor_value == 10 && !flashed.contains(&neighbor_pos) {
                to_flash.push(neighbor_pos);
                flashed.insert(neighbor_pos);
            }
        });
    }

    flashed
        .iter()
        .for_each(|p| *new_step.entry(*p).or_default() = 0);

    (new_step, flashed.len())
}

fn run(
    start: &PosMap,
    steps: usize,
    complete_func: Option<Box<dyn Fn(&PosMap, usize, usize, usize) -> bool>>,
) -> (usize, usize, bool) {
    let mut map = start.clone();

    let mut total = 0;
    let mut current = 0;

    let mut completed = complete_func.is_none();

    for i in 0..steps {
        let (next_map, flashes) = next_step(&map);

        total += flashes;
        map = next_map;
        current = i;

        if let Some(is_completed) = &complete_func {
            if is_completed(&map, total, flashes, current) {
                completed = true;
                break;
            }
        }
    }

    (total, current + 1, completed)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let map = load_map(&input)?;

    let (total, _, _) = run(&map, 100, None);

    println!("Part 1 | Total Flashes: {}", total);

    let (_, steps_needed, completed) = run(
        &map,
        10000,
        Some(Box::new(|map, _, last_flashes, _| {
            map.len() == last_flashes
        })),
    );

    if !completed {
        bail!("Part 2 | Complete Condition not met.");
    }

    println!("Part 2 | Steps Needed: {}", steps_needed);

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
    fn get_neighbors_test() {
        let map_raw = r"2199943210
        3987894921
        9856789892
        8767896789
        9899965678";

        let map = load_map(map_raw).unwrap();
        let neighbors = get_neighbors(&map, (2, 2));
        let set: HashSet<Point> = HashSet::from_iter(neighbors.iter().cloned());

        assert_eq!(set.len(), 8);
    }

    #[test]
    fn next_step_work_single() {
        let map_raw = r"11111
        19991
        19191
        19991
        11111";

        let start_map = load_map(map_raw).unwrap();
        let first_step = next_step(&start_map);
        println!("{:#?}", first_step)
    }
}
