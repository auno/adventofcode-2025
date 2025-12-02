use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<(usize,usize)>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .filter(|element| !element.is_empty())
        .map(|range| {
            let (a, b) = range.split_once('-').with_context(|| format!("Invalid range: {range}"))?;
            let a = a.parse().with_context(|| format!("Invalid range component: {a} (in {range})"))?;
            let b = b.parse().with_context(|| format!("Invalid range component: {b} (in {range})"))?;
            Ok((a, b))
        })
        .try_collect()
}

fn is_invalid(product_id: &usize) -> bool {
    let product_id = product_id.to_string();
    let len = product_id.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    product_id[0..(len / 2)] == product_id[(len / 2)..]
}

#[aoc(day2, part1)]
fn part1(ranges: &Input) -> usize {
    ranges
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(is_invalid)
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(1227775554, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(64215794229, part1(&parse(include_str!("../input/2025/day2.txt")).unwrap()));
    }
}
