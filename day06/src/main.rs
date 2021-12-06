use anyhow::{anyhow, bail, Result};

fn load_initial_population(input: &str) -> Result<[usize; 9]> {
    let mut population = [0; 9];

    let fish: Vec<usize> = input
        .split(",")
        .filter(|s| *s != "")
        .map(|s| {
            s.trim()
                .parse::<usize>()
                .map_err(|e| anyhow!("{}: {}", e, s))
        })
        .map(|r| match r {
            Ok(v) => match v {
                0..=8 => Ok(v),
                _ => bail!("invalid number {}", v),
            },
            Err(e) => Err(e),
        })
        .collect::<Result<Vec<usize>>>()?;

    fish.into_iter().for_each(|f| population[f] += 1);

    Ok(population)
}

fn next_population(start: &[usize; 9]) -> [usize; 9] {
    let mut next = start.clone();

    next.rotate_left(1);
    next[6] += next[8];

    next
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let initial_population = load_initial_population(&input)?;
    println!("{:?}", initial_population);

    let mut population_history: Vec<[usize; 9]> = Vec::new();
    population_history.push(initial_population.clone());

    let final_population = (0..256).into_iter().fold(initial_population, |current, _| {
        let next = next_population(&current);
        population_history.push(next.clone());

        next
    });

    let after_80: usize = population_history[80].iter().sum();
    let total: usize = final_population.iter().sum();

    println!("Part1 : {}", after_80);
    println!("Part2 : {}", total);

    Ok(())
}
