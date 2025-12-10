use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::path_finding::shortest_paths_to_target;

type IndicatorLights = Vec<bool>;
type WiringSchematics = Vec<Vec<usize>>;
type Joltages = Vec<usize>;

type Input = Vec<(IndicatorLights, WiringSchematics, Joltages)>;

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            let (target_lights, rest) = line.split_once(' ').with_context(|| format!("Invalid input: {line}"))?;
            let (button_wirings, joltage_requirements) = rest.rsplit_once(' ').with_context(|| format!("Invalid input: {line}"))?;

            let target_lights = target_lights
                .strip_prefix('[').with_context(|| format!("Invalid indicator light diagram: {target_lights} (in {line})"))?
                .strip_suffix(']').with_context(|| format!("Invalid indicator light diagram: {target_lights} (in {line})"))?
                .chars()
                .map(|c| match c {
                    '.' => Ok(false),
                    '#' => Ok(true),
                    _ => bail!("Invalid indicator light: {c} (in {line})"),
                })
                .try_collect()?;

            let button_wirings = button_wirings
                .split_ascii_whitespace()
                .map(|wiring| {
                    wiring
                        .strip_prefix('(').with_context(|| format!("Invalid wiring schematic: {wiring} (in {line})"))?
                        .strip_suffix(')').with_context(|| format!("Invalid wiring schematic: {wiring} (in {line})"))?
                        .split(',')
                        .map(|num| num.parse().with_context(|| format!("Invalid button: {num} (in {line})")))
                        .try_collect()
                })
                .try_collect()?;

            let joltage_requirements = joltage_requirements
                .strip_prefix('{').with_context(|| format!("Invalid joltage requirements: {joltage_requirements}"))?
                .strip_suffix('}').with_context(|| format!("Invalid joltage requirements: {joltage_requirements}"))?
                .split(',')
                .map(|num| num.parse().with_context(|| format!("Invalid joltage: {num} (in {line})")))
                .try_collect()?;

            Ok((target_lights, button_wirings, joltage_requirements))
        })
        .try_collect()
}

fn find_fewest_button_presses(target: &IndicatorLights, wirings: &WiringSchematics) -> Option<usize> {
    let (presses, _) = shortest_paths_to_target(
        vec![false; target.len()],
        |current| {
            wirings
                .iter()
                .map(|wiring| {
                    let neighbor = current
                        .iter()
                        .enumerate()
                        .map(|(i, light_state)| light_state ^ wiring.contains(&i))
                        .collect_vec();
                    (neighbor, 1)
                })
                .collect_vec()
        },
        |lights| lights == target,
    )?;

    Some(presses)
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> Option<usize> {
    input
        .iter()
        .map(|(target, wirings, _)| {
            find_fewest_button_presses(target, wirings)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(Some(7), part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(Some(409), part1(&parse(include_str!("../input/2025/day10.txt")).unwrap()));
    }
}
