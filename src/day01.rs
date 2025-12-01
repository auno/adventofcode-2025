use std::{iter::repeat_n, str::FromStr};

use anyhow::{bail, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl FromStr for Rotation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let Some((direction, distance)) = s.split_at_checked(1) else {
            bail!("Invalid Rotation: {s}");
        };

        let Ok(distance) = distance.parse() else {
            bail!("Invalid Rotation distance: {distance} (in {s})");
        };

        if distance < 0 {
            bail!("Invalid Rotation distance: {distance} (in {s})");
        }

        match direction {
            "L" => Ok(Rotation::Left(distance)),
            "R" => Ok(Rotation::Right(distance)),
            _ => bail!("Invalid Rotation direction: {direction} (in {s})"),
        }
    }
}

type Input = Vec<Rotation>;

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| line.parse())
        .try_collect()
}

fn sequence(rotations: &[Rotation]) -> Vec<i32> {
    rotations
        .iter()
        .fold(vec![50], |mut seq, rotation| {
            let position = seq.last().unwrap();
            let next_position = match rotation {
                Rotation::Left(distance) => (position - distance).rem_euclid(100),
                Rotation::Right(distance) => (position + distance).rem_euclid(100),
            };

            seq.push(next_position);
            seq
        })
}

#[aoc(day1, part1)]
fn part1(rotations: &Input) -> usize {
    sequence(rotations)
        .into_iter()
        .filter(|position| *position == 0)
        .count()
}

#[aoc(day1, part2)]
fn part2(rotations: &Input) -> usize {
    let rotations = rotations
        .iter()
        .flat_map(|rotation| {
            match rotation {
                Rotation::Left(distance) => repeat_n(Rotation::Left(1), *distance as usize),
                Rotation::Right(distance) => repeat_n(Rotation::Right(1), *distance as usize),
            }
        })
        .collect_vec();

    sequence(rotations.as_slice())
        .into_iter()
        .filter(|position| *position == 0)
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(3, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(6, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1021, part1(&parse(include_str!("../input/2025/day1.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(5933, part2(&parse(include_str!("../input/2025/day1.txt")).unwrap()));
    }
}
