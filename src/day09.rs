use std::cmp::{max, min};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::prelude::*;

type Point = (isize, isize);
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
fn part1(input: &Input) -> Option<isize> {
    input
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| {
            (max(p1.0, p2.0) - min(p1.0, p2.0) + 1) * (max(p1.1, p2.1) - min(p1.1, p2.1) + 1)
        })
        .max()
}

#[aoc(day9, part2)]
fn part2(corners: &Input) -> Option<isize> {
    corners
        .iter()
        .tuple_combinations()
        .par_bridge()
        .filter(|(p1, p2)| {
            let x_min = min(p1.0, p2.0);
            let x_max = max(p1.0, p2.0);
            let y_min = min(p1.1, p2.1);
            let y_max = max(p1.1, p2.1);

            corners
                .iter()
                .circular_tuple_windows()
                .all(|(q1, q2)| {
                    (min(q1.0, q2.0)..=max(q1.0, q2.0)).cartesian_product(min(q1.1, q2.1)..=max(q1.1, q2.1))
                        .all(|q| q.0 <= x_min || q.0 >= x_max || q.1 <= y_min || q.1 >= y_max)
                })
        })
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

    #[test]
    fn part2_example1() {
        assert_eq!(Some(24), part2(&parse(EXAMPLE1).unwrap()));
    }

    // #[test]
    // fn part2_input() {
    //     assert_eq!(Some(1470616992), part2(&parse(include_str!("../input/2025/day9.txt")).unwrap()));
    // }
}
