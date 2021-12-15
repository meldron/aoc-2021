use pathfinding::directed::dijkstra::dijkstra;
use std::collections::HashMap;

use anyhow::{anyhow, Result};

type Point = (isize, isize);
type Cavern = HashMap<Point, usize>;

pub fn load_map(input: &str) -> Result<Cavern> {
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

fn get_dim(input: &str) -> (usize, usize) {
    let y = input.lines().count();
    let x = input.lines().take(1).collect::<Vec<_>>()[0].len();

    (y, x)
}

fn get_destination(input: &str) -> Point {
    let (y, x) = get_dim(input);

    (y as isize - 1, x as isize - 1)
}

pub fn get_neighbors(map: &Cavern, point: Point) -> Vec<(Point, usize)> {
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

fn find_shortest_path(
    cavern: &Cavern,
    start: Point,
    destination: Point,
) -> Option<(Vec<Point>, usize)> {
    dijkstra(
        &start,
        |p: &Point| get_neighbors(cavern, *p),
        |p: &Point| *p == destination,
    )
}

fn expand_cavern(cavern: &Cavern, dimensions: (usize, usize), factor: usize) -> (Cavern, Point) {
    let mut expanded = Cavern::new();

    let new_y_size = dimensions.0 * factor;
    let new_x_size = dimensions.1 * factor;

    (0..new_y_size).for_each(|y| {
        (0..new_x_size).for_each(|x| {
            let y_factor = y / dimensions.0;
            let x_factor = x / dimensions.1;

            let y_pos = (y % dimensions.0) as isize;
            let x_pos = (x % dimensions.1) as isize;

            let cost_org = cavern.get(&(y_pos, x_pos)).unwrap();
            let mut cost_new = cost_org + y_factor + x_factor;
            if cost_new > 9 {
                cost_new -= 9;
            }

            expanded.insert((y as isize, x as isize), cost_new);
        })
    });

    let destination = (new_y_size as isize - 1, new_x_size as isize - 1);

    (expanded, destination)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let cavern = load_map(&input)?;
    let start: Point = (0, 0);
    let destination_1 = get_destination(&input);

    let shortest_path_p1 =
        find_shortest_path(&cavern, start, destination_1).ok_or(anyhow!("no path found"))?;
    println!("P1: {}", shortest_path_p1.1);

    let dimensions = get_dim(&input);

    let (expanded_cavern, expanded_destination) = expand_cavern(&cavern, dimensions, 5);
    let shortest_path_p2 = find_shortest_path(&expanded_cavern, start, expanded_destination)
        .ok_or(anyhow!("no path found"))?;
    println!("P2: {}", shortest_path_p2.1);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_map_working() {
        let map_raw = r"1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";

        let cavern = load_map(map_raw).expect("");
        let start: Point = (0, 0);
        let destination = get_destination(map_raw);
        let shortest_path = find_shortest_path(&cavern, start, destination).unwrap();
        assert_eq!(shortest_path.1, 40);
    }
}
