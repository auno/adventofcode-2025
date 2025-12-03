use std::cmp::max;

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

fn largest_joltage(bank: &[Joltage]) -> Joltage {
    let mut best = 0;
    let mut best_battery = 0;

    for &battery in bank {
        let current = best_battery * 10 + battery;
        best = max(current, best);
        best_battery = max(battery, best_battery);
    }

    best
}

#[aoc(day3, part1)]
fn part1(input: &Input) -> Joltage {
    input
        .iter()
        .map(|bank| largest_joltage(bank))
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
    fn largest_joltage_example1_bank1() {
        assert_eq!(98, largest_joltage(&parse_bank("987654321111111").unwrap()));
    }

    #[test]
    fn largest_joltage_example1_bank2() {
        assert_eq!(89, largest_joltage(&parse_bank("811111111111119").unwrap()));
    }

    #[test]
    fn largest_joltage_example1_bank3() {
        assert_eq!(78, largest_joltage(&parse_bank("234234234234278").unwrap()));
    }

    #[test]
    fn largest_joltage_example1_bank4() {
        assert_eq!(92, largest_joltage(&parse_bank("818181911112111").unwrap()));
    }

    #[test]
    fn part1_example1() {
        assert_eq!(357, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(17278, part1(&parse(include_str!("../input/2025/day3.txt")).unwrap()));
    }
}
