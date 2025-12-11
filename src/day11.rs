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

fn reverse(connections: &Input) -> Input {
    let mut reverse_connections: Input = HashMap::new();

    for (device, outputs) in connections {
        for output in outputs {
            reverse_connections.entry(output.to_string()).or_default().push(device.to_string());
        }
    }

    reverse_connections
}

fn count_paths(connections: &Input, reverse_connections: &Input, source: &str, target: &str) -> Option<usize> {
    let forward_reachable = find_reachable(connections, source);
    let reverse_reachable = find_reachable(reverse_connections, target);
    let reachable_between = forward_reachable.intersection(&reverse_reachable).copied().collect::<HashSet<_>>();

    let connections = prune_unreachable(connections, &reachable_between);
    let reverse_connections = prune_unreachable(reverse_connections, &reachable_between);

    let mut num_paths: HashMap<_, _> = HashMap::from([(target, 1)]);
    let mut num_connections_processed: HashMap<&str, usize> = HashMap::new();
    let mut queue = VecDeque::from([target]);
    let mut processed: HashSet<_> = HashSet::new();

    while let Some(current) = queue.pop_front() {
        if processed.contains(current) {
            continue;
        }

        if num_connections_processed.get(current).copied().unwrap_or_default() == connections.get(current).map(Vec::len).unwrap_or_default() {
            let current_num_paths = num_paths.get(current).copied().unwrap_or_default();

            if let Some(current_reverse_connections) = reverse_connections.get(current) {
                for reverse_connection in current_reverse_connections {
                    *num_paths.entry(reverse_connection).or_default() += current_num_paths;
                    *num_connections_processed.entry(reverse_connection).or_default() += 1;
                    queue.push_back(reverse_connection);
                }
            }

            processed.insert(current);
        }
    }

    num_paths.get(source).copied()
}

#[aoc(day11, part1)]
fn part1(connections: &Input) -> Option<usize> {
    let reverse_connections = reverse(connections);
    count_paths(connections, &reverse_connections, "you", "out")
}

fn find_reachable<'a>(connections: &'a Input, source: &'a str) -> HashSet<&'a str> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([source]);

    while let Some(current) = queue.pop_front() {
        if seen.insert(current) {
            let Some(current_connections) = connections.get(current) else {
                continue;
            };

            for connection in current_connections {
                queue.push_back(connection);
            }
        }
    }

    seen
}

fn prune_unreachable(connections: &Input, reachable: &HashSet<&str>) -> Input {
    connections
        .iter()
        .filter(|(device, _)| reachable.contains(device.as_str()))
        .map(|(device, outputs)| (
            device.to_string(),
            outputs.iter().filter(|output| reachable.contains(output.as_str())).cloned().collect(),
        ))
        .collect()
}

#[aoc(day11, part2)]
fn part2(connections: &Input) -> Option<usize> {
    let reverse_connections = reverse(connections);

    let reachable_from_dac = find_reachable(connections, "dac");
    let reachable_from_fft = find_reachable(connections, "fft");

    let order = if reachable_from_dac.contains("fft") {
        [ "svr", "dac", "fft", "out" ]
    } else if reachable_from_fft.contains("dac") {
        [ "svr", "fft", "dac", "out" ]
    } else {
        return None;
    };

    order
        .into_iter()
        .tuple_windows()
        .map(|(source, target)| count_paths(connections, &reverse_connections, source, target))
        .product()
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

    const EXAMPLE2: &str = indoc! {"
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(Some(5), part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(Some(658), part1(&parse(include_str!("../input/2025/day11.txt")).unwrap()));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(Some(2), part2(&parse(EXAMPLE2).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(Some(371113003846800), part2(&parse(include_str!("../input/2025/day11.txt")).unwrap()));
    }
}
