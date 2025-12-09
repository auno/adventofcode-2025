use std::cmp::{min, max};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (usize, usize);
type Input = Vec<Point>;

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').with_context(|| format!("Invalid coordinates: {line}"))?;
            let x = x.parse().with_context(|| format!("Invalid coordinate: {x} (in {line})"))?;
            let y = y.parse().with_context(|| format!("Invalid coordinate: {x} (in {line})"))?;

            Ok((x, y))
        })
        .try_collect()
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> Option<usize> {
    input
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| {
            (max(p1.0, p2.0) - min(p1.0, p2.0) + 1) * (max(p1.1, p2.1) - min(p1.1, p2.1) + 1)
        })
        .max()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(Some(50), part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(Some(4781377701), part1(&parse(include_str!("../input/2025/day9.txt")).unwrap()));
    }
}
