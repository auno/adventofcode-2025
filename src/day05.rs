use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = (Vec<(usize, usize)>, Vec<usize>);

#[aoc_generator(day5)]
fn parse(input: &str) -> Result<Input> {
    let Some((fresh_ranges, available)) = input.split_once("\n\n") else {
        bail!("Malformed input");
    };

    let fresh_ranges: Result<Vec<(usize, usize)>> = fresh_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').with_context(|| format!("Invalid ingredient ID range: {line}"))?;
            let start = start.parse::<usize>().with_context(|| format!("Invalid ingredient ID: {start} (in {start}-{end})"))?;
            let end = end.parse::<usize>().with_context(|| format!("Invalid ingredient ID: {end} (in {start}-{end})"))?;
            Ok((start, end))
        })
        .try_collect();

    let available = available
        .lines()
        .map(|line| line.parse().with_context(|| format!("Invalid ingredient ID: {line}")))
        .try_collect();

    Ok((fresh_ranges?, available?))
}

#[aoc(day5, part1)]
fn part1((fresh_ranges, available): &Input) -> usize {
    available
        .iter()
        .filter(|id| fresh_ranges.iter().any(|(start, end)| *id >= start && *id <= end))
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(3, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(770, part1(&parse(include_str!("../input/2025/day5.txt")).unwrap()));
    }
}
