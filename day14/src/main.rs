use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};

type RuleBook = HashMap<([char; 2]), char>;
type Template = Vec<char>;

fn get_rule_book(raw: &str) -> Result<RuleBook> {
    raw.lines()
        .map(|l| {
            let (from_raw, to_raw) = l
                .trim()
                .split_once(" -> ")
                .ok_or(anyhow!("invalid rule: {}", l))?;

            let from: Result<([char; 2]), _> =
                TryInto::try_into(from_raw.trim().chars().collect::<Vec<_>>());

            let to = to_raw.trim().chars().take(1).last();

            match (from, to) {
                (Ok(f), Some(t)) => Ok((f, t)),
                _ => bail!("invalid rule (from or to): {}", l),
            }
        })
        .collect()
}

fn load_input(input: &str) -> Result<(Template, RuleBook)> {
    let (template_raw, rules_raw) = input
        .trim()
        .split_once("\n\n")
        .ok_or(anyhow!("invalid input"))?;

    let template = template_raw.trim().chars().collect();
    let rule_book = get_rule_book(&rules_raw)?;

    Ok((template, rule_book))
}

fn apply_template_p1(template: &Template, rule_book: &RuleBook) -> Result<Template> {
    let next = template
        .windows(2)
        .map(|rule| {
            let start = rule.first().unwrap();
            let middle = rule_book.get(rule).ok_or(anyhow!("rule not found"))?;
            Ok(vec![*start, *middle])
        })
        .collect::<Result<Vec<Vec<char>>>>()?;

    let mut new_template = next.into_iter().fold(Vec::new(), |mut template, test| {
        test.iter().for_each(|c| template.push(*c));
        template
    });

    new_template.push(*template.last().unwrap());

    Ok(new_template)
}

fn max_min_diff(template: &Template) -> Result<usize> {
    let mut map: HashMap<char, usize> = HashMap::new();

    template
        .iter()
        .for_each(|c| *map.entry(*c).or_default() += 1);

    let max = map.values().max().ok_or(anyhow!("max not found"))?;
    let min = map.values().min().ok_or(anyhow!("max not found"))?;

    println!("{} {}", max, min);

    Ok(*max - *min)
}

fn run(template: Template, rule_book: &RuleBook, steps: usize) -> Result<Template> {
    (0..steps).into_iter().try_fold(template, |current, _| {
        apply_template_p1(&current, &rule_book)
    })
}

fn run_p2(template: Template, rule_book: &RuleBook, steps: usize) -> usize {
    let mut start: HashMap<[char; 2], usize> = HashMap::new();

    template.windows(2).for_each(|rule| {
        *start.entry(rule.try_into().unwrap()).or_default() += 1;
    });

    let done = (0..steps).fold(start, |current, _| {
        let mut next = HashMap::new();

        current.iter().for_each(|(pair, count)| {
            if let Some(result) = rule_book.get(pair) {
                *next.entry([pair[0], *result]).or_default() += count;
                *next.entry([*result, pair[1]]).or_default() += count;
            }
        });

        next
    });

    let mut poly_counter =
        done.iter()
            .fold(HashMap::<char, usize>::new(), |mut map, (pair, count)| {
                *map.entry(pair[1]).or_default() += count;
                map
            });
    *poly_counter.entry(template[0]).or_default() += 1;

    let max = poly_counter.values().max().unwrap();
    let min = poly_counter.values().min().unwrap();

    max - min
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let (template_start, rule_book) = load_input(&input)?;

    let template_after_10_steps = run(template_start.clone(), &rule_book, 10)?;
    let diff_p1 = max_min_diff(&template_after_10_steps)?;

    println!("Part 1: {}", diff_p1);

    let diff_p2 = run_p2(template_start.clone(), &rule_book, 40);

    println!("Part 2: {}", diff_p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_RULES: &str = r"CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    #[test]
    fn get_rule_book_working() {
        let rule_book = get_rule_book(EXAMPLE_RULES).unwrap();
        assert_eq!(rule_book.len(), 16)
    }

    #[test]
    fn test_run() {
        let rule_book = get_rule_book(EXAMPLE_RULES).unwrap();
        let template = vec!['N', 'N', 'C', 'B'];

        let new_template = run(template, &rule_book, 2).unwrap();
        assert_eq!(new_template.len(), 13);
    }

    #[test]
    fn run_p2_working() {
        let rule_book = get_rule_book(EXAMPLE_RULES).unwrap();
        let template = vec!['N', 'N', 'C', 'B'];

        let new_template = run_p2(template, &rule_book, 2);
        assert_eq!(new_template, 5);
    }
}
