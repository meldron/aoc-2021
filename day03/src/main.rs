use anyhow::{anyhow, bail, Result};

pub trait FromBinaryString {
    fn from_binary_str(s: &str) -> Result<Self>
    where
        Self: Sized;

    fn is_bit_set(&self, n: usize, num_bits: usize) -> bool;
}

impl FromBinaryString for i32 {
    fn from_binary_str(s: &str) -> Result<i32> {
        i32::from_str_radix(s, 2).map_err(|e| anyhow!(e))
    }

    fn is_bit_set(&self, n: usize, num_bits: usize) -> bool {
        if n > 11 {
            panic!("is_bit_set_12 with value > 11")
        }

        let mask = 1 << (num_bits - 1 - n);

        self & mask != 0
    }
}

fn calc_cols(lines: &[String]) -> [i32; 12] {
    let mut cols = [0; 12];

    lines.iter().for_each(|l| {
        l.chars().enumerate().for_each(|(i, c)| match c {
            '1' => cols[i] += 1,
            '0' => {}
            _ => panic!("unknown bit"),
        })
    });

    cols
}

fn power_consumption(lines: &[String]) -> i32 {
    let cols = calc_cols(lines);

    let (gamma, epsilon) =
        cols.iter()
            .rev()
            .enumerate()
            .fold((0, 0), |(mut gamma, mut epsilon), (i, v)| {
                let d = *v as f32 / lines.len() as f32;

                let mask = 1 << i;

                if d >= 0.5 {
                    gamma |= mask;
                } else {
                    epsilon |= mask;
                }

                (gamma, epsilon)
            });

    gamma * epsilon
}

fn life_system_rating(lines: &[String], co2: bool) -> Result<i32> {
    let all_numbers: Vec<i32> = lines
        .iter()
        .map(|s| i32::from_binary_str(s))
        .collect::<Result<Vec<i32>>>()?;

    let num_bits = lines[0].len();

    let left = (0..num_bits).scan(all_numbers, |left, bit| {
        if left.len() <= 1 {
            return None;
        }

        let bits_set = left.iter().filter(|v| v.is_bit_set(bit, num_bits)).count();

        let d = bits_set as f32 / left.len() as f32;

        let is_set_filter = match co2 {
            true => d < 0.5,
            false => d >= 0.5,
        };

        let remaining: Vec<i32> = left
            .iter()
            .filter(|v| v.is_bit_set(bit, num_bits) == is_set_filter)
            .map(|v| *v)
            .collect();

        *left = remaining;

        Some(left.clone())
    });

    match left.last() {
        Some(last) => last.get(0).map(|x| *x).ok_or(anyhow!("not last value")),
        None => bail!("no last value (iterator)"),
    }
}

fn main() -> Result<()> {
    let lines: Vec<String> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.to_owned())
        .collect();

    let power = power_consumption(&lines);

    println!("Power Consumption: {}", power);

    let oxygen = life_system_rating(&lines, false)?;
    println!("oxygen: {:?}", oxygen);

    let co2 = life_system_rating(&lines, true)?;
    println!("co2: {:?}", co2);

    let life_support_rating = co2 * oxygen;
    println!("life_support_rating: {:?}", life_support_rating);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binary_number() {
        let bs_2044 = "011111111100";
        let parsed = i32::from_binary_str(bs_2044);

        assert_eq!(parsed.unwrap(), 2044)
    }

    #[test]
    fn is_bit_set() {
        let bs_2044 = "011111111100";
        let parsed = i32::from_binary_str(bs_2044).expect("");

        let first_bit = parsed.is_bit_set(0, 12);
        let second_bit = parsed.is_bit_set(1, 12);

        assert_eq!(first_bit, false);
        assert_eq!(second_bit, true);
    }

    #[test]
    fn life_support() {
        let input = r#"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010"#;

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_owned()).collect();

        let oxygen = life_system_rating(&lines, false).unwrap();
        assert_eq!(oxygen, 23);

        let co2 = life_system_rating(&lines, true).unwrap();
        assert_eq!(co2, 10);

        assert_eq!(oxygen * co2, 230)
    }
}
