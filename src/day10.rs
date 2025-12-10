use std::iter::Sum;

use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use z3::{ast::Int, Optimize};
use rayon::prelude::*;

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

#[aoc(day10, part1)]
fn part1(input: &Input) -> Option<usize> {
    input
        .iter()
        .map(|(target, wirings, _)| {
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
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &Input) -> Option<u64> {
    input
        .par_iter()
        .map(|(_, wirings, target)| {
            let vars = (0..(wirings.len()))
                .map(|_| Int::fresh_const("button"))
                .collect_vec();

            let optimizer = Optimize::new();

            for var in &vars {
                optimizer.assert(&var.ge(0));
            }

            for (i, target_value) in target.iter().enumerate() {
                optimizer.assert(
                    &Int::sum(
                        (0..(wirings.len()))
                            .filter(|j| {
                                wirings
                                    .get(*j)
                                    .map(|wiring| wiring.contains(&i))
                                    .unwrap_or_default()
                            })
                            .map(|j| &vars[j])
                    ).eq(*target_value as u64)
                )
            }

            optimizer.minimize(&Int::sum(vars.iter()));
            optimizer.check(&[]);

            let model = optimizer.get_model().unwrap();
            vars
                .iter()
                .map(|var| model.eval(var, true).map(|x| x.as_u64()))
                .map(|x| x.flatten())
                .sum::<Option<u64>>()
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

    #[test]
    fn part2_example1() {
        assert_eq!(Some(33), part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(Some(15489), part2(&parse(include_str!("../input/2025/day10.txt")).unwrap()));
    }
}
