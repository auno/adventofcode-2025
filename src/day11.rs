use std::collections::{HashSet, VecDeque};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use itertools::Itertools;

type Input = HashMap<String, Vec<String>>;

#[aoc_generator(day11)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            let (device, outputs) = line.split_once(": ").with_context(|| format!("Invalid input line: {line}"))?;
            let device = device.to_string();
            let outputs = outputs
                .split_ascii_whitespace()
                .map(&str::to_string)
                .collect_vec();

            Ok((device, outputs))
        })
        .try_collect()
}

fn reverse(connections: &Input) -> HashMap<&str, Vec<&str>> {
    let mut reverse_connections: HashMap<&str, Vec<&str>> = HashMap::new();

    for (device, outputs) in connections {
        for output in outputs {
            reverse_connections.entry(output).or_default().push(device);
        }
    }

    reverse_connections
}

#[aoc(day11, part1)]
fn part1(connections: &Input) -> Option<usize> {
    let reverse_connections = reverse(connections);

    let mut num_paths: HashMap<_, _> = HashMap::from([("out", 1)]);
    let mut num_connections_processed: HashMap<&str, usize> = HashMap::new();
    let mut queue = VecDeque::from(["out"]);
    let mut processed: HashSet<_> = HashSet::new();

    while let Some(current) = queue.pop_front() {
        if processed.contains(current) {
            continue;
        }

        if num_connections_processed.get(current).copied().unwrap_or_default() == connections.get(current).map(Vec::len).unwrap_or_default() {
            let current_num_paths = num_paths.get(current).copied().unwrap_or_default();

            for reverse_connection in reverse_connections.get(current).unwrap_or(&vec![]) {
                *num_paths.entry(reverse_connection).or_default() += current_num_paths;
                *num_connections_processed.entry(reverse_connection).or_default() += 1;
                queue.push_back(reverse_connection);
            }

            processed.insert(current);
        }
    }

    num_paths.get("you").copied()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(Some(5), part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(Some(658), part1(&parse(include_str!("../input/2025/day11.txt")).unwrap()));
    }
}
