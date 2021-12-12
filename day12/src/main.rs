use anyhow::{anyhow, bail, Error, Result};
use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Hash, PartialEq, Eq)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl std::fmt::Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cave::Start => write!(f, "start"),
            Cave::End => write!(f, "end"),
            Cave::Big(s) => write!(f, "{}", s),
            Cave::Small(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let cave = match s.trim() {
            "start" => Cave::Start,
            "end" => Cave::End,
            _ => {
                if s.chars().all(|c| c.is_uppercase()) {
                    Cave::Big(s.to_owned())
                } else if s.chars().all(|c| c.is_lowercase()) {
                    Cave::Small(s.to_owned())
                } else {
                    bail!("Neither small nor big cave: {}", s)
                }
            }
        };

        Ok(cave)
    }
}

#[derive(Debug, Clone)]
struct CaveNetwork {
    network: HashMap<Cave, Vec<Cave>>,
}

impl FromStr for CaveNetwork {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut network: HashMap<Cave, Vec<Cave>> = HashMap::new();

        (s.trim().lines().try_for_each(|line| {
            let split: Vec<&str> = line.split("-").map(|s| s.trim()).collect();
            let left = split.get(0).ok_or(anyhow!("Left cave not found"))?;
            let right = split.get(1).ok_or(anyhow!("Right cave not found"))?;

            let left_cave = Cave::from_str(&left)?;
            let right_cave = Cave::from_str(&right)?;

            network
                .entry(left_cave.clone())
                .or_default()
                .push(right_cave.clone());
            network.entry(right_cave).or_default().push(left_cave);

            Ok(())
        }) as Result<()>)?;

        Ok(Self { network })
    }
}

struct CavePathFinder {
    pub visited: Vec<Cave>,
    pub connections: Vec<Cave>,
    pub small_caves_visited: HashMap<Cave, usize>,
}

impl CaveNetwork {
    pub fn all_paths(
        &self,
        single_small_cave_can_be_visited_twice: bool,
    ) -> Result<Vec<Vec<Cave>>> {
        let mut paths = Vec::<Vec<Cave>>::new();

        let start_connections = self
            .network
            .get(&Cave::Start)
            .ok_or(anyhow!("start not found"))?;

        let mut path_finder: Vec<CavePathFinder> = vec![CavePathFinder {
            visited: vec![Cave::Start],
            connections: start_connections.clone(),
            small_caves_visited: HashMap::new(),
        }];

        while let Some(pf) = path_finder.pop() {
            pf.connections.iter().for_each(|cave| {
                let mut small_caves_visited = pf.small_caves_visited.clone();
                if matches!(cave, Cave::Small(_)) {
                    *small_caves_visited.entry(cave.clone()).or_default() += 1;
                }

                let visited: Vec<Cave> = pf
                    .visited
                    .clone()
                    .into_iter()
                    .chain(vec![cave.clone()].into_iter())
                    .collect();

                let no_small_cave_visited_twice =
                    small_caves_visited.values().filter(|f| **f > 1).count() == 0;

                let visited_filter: Box<dyn Fn(&Cave) -> bool> =
                    match single_small_cave_can_be_visited_twice {
                        true => Box::new(|to_check| {
                            !matches!(to_check, Cave::Small(_))
                                || !small_caves_visited.contains_key(to_check)
                                || no_small_cave_visited_twice
                        }),
                        false => Box::new(|to_check| !small_caves_visited.contains_key(to_check)),
                    };

                let connections = self
                    .network
                    .get(cave)
                    .expect("cave not found")
                    .clone()
                    .into_iter()
                    .filter(|f| !matches!(f, Cave::Start))
                    .filter(visited_filter)
                    .collect();

                match cave {
                    Cave::End => paths.push(visited),
                    _ => {
                        path_finder.push(CavePathFinder {
                            connections,
                            small_caves_visited,
                            visited,
                        });
                    }
                }
            });
        }

        Ok(paths)
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let network = CaveNetwork::from_str(&input)?;

    let all_paths_1 = network.all_paths(false)?;
    println!("Part1 | num paths: {}", all_paths_1.len());

    let all_paths_2 = network.all_paths(true)?;
    println!("Part2 | num paths: {}", all_paths_2.len());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_cave_start() {
        assert_eq!(Cave::from_str("start").unwrap(), Cave::Start);
    }

    #[test]
    fn parse_cave_end() {
        assert_eq!(Cave::from_str("end").unwrap(), Cave::End);
    }

    #[test]
    fn parse_cave_small() {
        assert_eq!(Cave::from_str("bk").unwrap(), Cave::Small("bk".to_owned()));
    }

    #[test]
    fn parse_cave_big() {
        assert_eq!(Cave::from_str("BK").unwrap(), Cave::Big("BK".to_owned()));
    }

    #[test]
    fn parse_cave_invalid() {
        assert_eq!(
            Cave::from_str("bK").unwrap_err().to_string(),
            anyhow!("Neither small nor big cave: bK").to_string()
        );
    }

    static TEST_NETWORK_SMALL: &str = r"start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";

    static TEST_NETWORK_MEDIUM: &str = r"dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc";

    #[test]
    fn path_network_small() {
        let network = CaveNetwork::from_str(TEST_NETWORK_SMALL).unwrap();

        let paths = network.all_paths(true).unwrap();

        assert_eq!(paths.len(), 36)
    }

    #[test]
    fn path_network_medium() {
        let network = CaveNetwork::from_str(TEST_NETWORK_MEDIUM).unwrap();

        let paths = network.all_paths(true).unwrap();

        assert_eq!(paths.len(), 103)
    }
}
