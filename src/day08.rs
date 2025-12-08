use std::collections::VecDeque;

use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Point = (isize, isize, isize);
type Input = Vec<Point>;

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<_> = line
                .split(',')
                .map(|num| num.parse().with_context(|| format!("Invalid coordinate: {num} (in {line})")))
                .try_collect()?;

            match coords[..] {
                [x, y, z] => Ok((x, y, z)),
                _ => bail!("Wrong number of coordinates: {line}"),
            }
        })
        .try_collect()
}

fn normalize_pair(a: Point, b: Point) -> (Point, Point) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

fn find_distances(points: &[Point]) -> HashMap<(Point, Point), f64> {
    let mut distances: HashMap<(Point, Point), f64> = HashMap::new();

    for (p1, p2) in points.iter().tuple_combinations() {
        let distance = (((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)) as f64).sqrt();
        let pair = normalize_pair(*p1, *p2);
        distances.insert(pair, distance);
    }

    distances
}

fn find_edges_by_distance(distances: &HashMap<(Point, Point), f64>) -> Vec<(Point, Point)> {
    distances
        .into_iter()
        .sorted_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|((p1, p2), _)| (*p1, *p2))
        .collect_vec()
}

fn find_circuits(
    points: &[(isize, isize, isize)],
    edges: &[(Point, Point)],
) -> Vec<HashSet<(isize, isize, isize)>> {
    let mut connections: HashMap<Point, Vec<Point>> = HashMap::new();

    for &(p1, p2) in edges {
        connections.entry(p1).or_default().push(p2);
        connections.entry(p2).or_default().push(p1);
    }

    let mut circuits = vec![];
    let mut processed: HashSet<Point> = HashSet::new();

    for &p in points {
        if processed.contains(&p) { continue; }

        let mut circuit: HashSet<Point> = HashSet::from([p]);
        let mut queue = VecDeque::from([p]);

        while let Some(q) = queue.pop_front() {
            let Some(rs) = connections.get(&q) else { continue; };

            for &r in rs {
                if !circuit.contains(&r) {
                    queue.push_back(r);
                    circuit.insert(r);
                }
            }
        }

        processed.extend(circuit.iter().copied());
        circuits.push(circuit);
    }

    circuits
}

fn solve1(points: &Input, num_pairs: usize) -> usize {
    let distances = find_distances(points);

    let edges = find_edges_by_distance(&distances)
        .into_iter()
        .take(num_pairs)
        .collect_vec();

    let circuits = find_circuits(points, &edges);

    circuits
        .into_iter()
        .map(|circuit| circuit.len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    solve1(input, 1000)
}

#[aoc(day8, part2)]
fn part2(points: &Input) -> Option<isize> {
    let distances = find_distances(points);
    let edges = find_edges_by_distance(&distances);

    for i in 1..edges.len() {
        if find_circuits(points, &edges[0..i]).len() == 1 {
            let (p1, p2) = edges[i - 1];
            return Some(p1.0 * p2.0);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(40, solve1(&parse(EXAMPLE1).unwrap(), 10));
    }

    #[test]
    fn part1_input() {
        assert_eq!(80446, part1(&parse(include_str!("../input/2025/day8.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(Some(25272), part2(&parse(EXAMPLE1).unwrap()));
    }

    // #[test]
    // fn part2_input() {
    //     assert_eq!(Some(51294528), part2(&parse(include_str!("../input/2025/day8.txt")).unwrap()));
    // }
}
