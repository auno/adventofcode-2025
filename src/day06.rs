use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{multizip, Itertools};

enum Operation {
    Addition,
    Multiplication,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Addition),
            "*" => Ok(Self::Multiplication),
            _ => bail!("Invalid operation: {s}"),
        }
    }
}

type Input = (Vec<Vec<u64>>, Vec<Operation>);

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Input> {
    let lines = input.lines().collect_vec();

    let numbers = lines[0..(lines.len() - 1)]
        .iter()
        .map(|line| {
            line.trim()
                .split_ascii_whitespace()
                .map(|num| num.parse().with_context(|| format!("Invalid number: {num}")))
                .try_collect()
        })
        .try_collect()?;

    let operations = lines[lines.len() - 1]
        .trim()
        .split_ascii_whitespace()
        .map(|op| op.parse())
        .try_collect()?;

    Ok((numbers, operations))
}

#[aoc(day6, part1)]
fn part1((numbers, operations): &Input) -> u64 {
    let init = operations
        .iter()
        .map(|op| match op {
            Operation::Addition => 0,
            Operation::Multiplication => 1,
        })
        .collect_vec();

    numbers
        .iter()
        .fold(init, |acc, nums| {
            multizip((acc, nums, operations))
                .map(|(a, b, op)| {
                    match op {
                        Operation::Addition => a + b,
                        Operation::Multiplication => a * b,
                    }
                })
                .collect_vec()
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(4277556, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(5733696195703, part1(&parse(include_str!("../input/2025/day6.txt")).unwrap()));
    }
}
