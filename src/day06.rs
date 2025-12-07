use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

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

#[aoc_generator(day6, part1)]
fn parse1(input: &str) -> Result<Input> {
    let lines = input.lines().collect_vec();

    let numbers: Vec<Vec<u64>> = lines[0..(lines.len() - 1)]
        .iter()
        .map(|line| {
            line.trim()
                .split_ascii_whitespace()
                .map(|num| num.parse().with_context(|| format!("Invalid number: {num}")))
                .try_collect()
        })
        .try_collect()?;

    let mut numbers_transposed = vec![vec![0; numbers.len()]; numbers[0].len()];

    for i in 0..numbers.len() {
        for j in 0..numbers[0].len() {
            numbers_transposed[j][i] = numbers[i][j];
        }
    }

    let operations = lines[lines.len() - 1]
        .trim()
        .split_ascii_whitespace()
        .map(|op| op.parse())
        .try_collect()?;

    Ok((numbers_transposed, operations))
}

#[aoc_generator(day6, part2)]
fn parse2(input: &str) -> Result<Input> {
    let lines = input.lines().collect_vec();

    let numbers_transposed = &lines[0..(lines.len() - 1)]
        .iter()
        .fold(vec![vec![]; lines[0].len()], |mut acc, line| {
            for (j, c) in line.chars().rev().enumerate() {
                acc[j].push(c);
            }

            acc
        })
        .iter()
        .map(|line| String::from_iter(line.iter()))
        .collect_vec();

    let numbers = numbers_transposed
        .split(|line| line.trim().is_empty())
        .map(|number_group| {
            number_group
                .iter()
                .map(|number| number.trim().parse().with_context(|| format!("Invalid number: {number}")))
                .try_collect()
        })
        .try_collect()?;

    let operations = lines[lines.len() - 1]
        .trim()
        .split_ascii_whitespace()
        .rev()
        .map(|op| op.parse())
        .try_collect()?;

    Ok((numbers, operations))
}

#[aoc(day6, part1)]
#[aoc(day6, part2)]
fn solve((numbers, operations): &Input) -> u64 {
    numbers
        .iter()
        .zip(operations)
        .map(|(number_group, operation)| {
            let init = match operation {
                Operation::Addition => 0,
                Operation::Multiplication => 1,
            };

            number_group
                .iter()
                .fold(init, |acc, num| {
                    match operation {
                        Operation::Addition => acc + num,
                        Operation::Multiplication => acc * num,
                    }
                })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  \n",
    );

    #[test]
    fn part1_example1() {
        assert_eq!(4277556, solve(&parse1(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(5733696195703, solve(&parse1(include_str!("../input/2025/day6.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(3263827, solve(&parse2(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(10951882745757, solve(&parse2(include_str!("../input/2025/day6.txt")).unwrap()));
    }
}
