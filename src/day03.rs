use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Joltage = usize;
type Bank = Vec<Joltage>;
type Input = Vec<Bank>;

fn parse_bank(bank: &str) -> Result<Bank> {
    (0..(bank.len()))
        .map(|i| &bank[i..(i + 1)])
        .map(|digit| digit.parse().with_context(|| "Invalid battery joltage: {digit} (in bank {line})"))
        .try_collect()
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(parse_bank)
        .try_collect()
}

fn largest_joltage(bank: &[Joltage], num_batteries: usize) -> Joltage {
    let mut offset = 0;
    let mut joltages = vec![];

    for batteries_left in (1..=num_batteries).rev() {
        let (new_offset, joltage) = bank
            .iter()
            .enumerate()
            .take(bank.len() - (batteries_left - 1))
            .skip(offset)
            .rev()
            .max_by_key(|(_, joltage)| **joltage)
            .unwrap();

        joltages.push(*joltage);
        offset = new_offset + 1;
    }

    joltages
        .into_iter()
        .fold(0, |acc, v| acc * 10 + v)
}

#[aoc(day3, part1)]
fn part1(input: &Input) -> Joltage {
    input
        .iter()
        .map(|bank| largest_joltage(bank, 2))
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Input) -> Joltage {
    input
        .iter()
        .map(|bank| largest_joltage(bank, 12))
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn largest_joltage_example1_bank1_2() {
        assert_eq!(98, largest_joltage(&parse_bank("987654321111111").unwrap(), 2));
    }

    #[test]
    fn largest_joltage_example1_bank2_2() {
        assert_eq!(89, largest_joltage(&parse_bank("811111111111119").unwrap(), 2));
    }

    #[test]
    fn largest_joltage_example1_bank3_2() {
        assert_eq!(78, largest_joltage(&parse_bank("234234234234278").unwrap(), 2));
    }

    #[test]
    fn largest_joltage_example1_bank4_2() {
        assert_eq!(92, largest_joltage(&parse_bank("818181911112111").unwrap(), 2));
    }

    #[test]
    fn largest_joltage_example1_bank1_12() {
        assert_eq!(987654321111, largest_joltage(&parse_bank("987654321111111").unwrap(), 12));
    }

    #[test]
    fn largest_joltage_example1_bank2_12() {
        assert_eq!(811111111119, largest_joltage(&parse_bank("811111111111119").unwrap(), 12));
    }

    #[test]
    fn largest_joltage_example1_bank3_12() {
        assert_eq!(434234234278, largest_joltage(&parse_bank("234234234234278").unwrap(), 12));
    }

    #[test]
    fn largest_joltage_example1_bank4_12() {
        assert_eq!(888911112111, largest_joltage(&parse_bank("818181911112111").unwrap(), 12));
    }

    #[test]
    fn part1_example1() {
        assert_eq!(357, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(17278, part1(&parse(include_str!("../input/2025/day3.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(3121910778619, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(171528556468625, part2(&parse(include_str!("../input/2025/day3.txt")).unwrap()));
    }
}
