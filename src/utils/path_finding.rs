#![allow(dead_code)]

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};
use std::hash::Hash;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Distances<SearchNode> = HashMap<SearchNode, (usize, Vec<SearchNode>)>;
pub type PathMap<SearchNode> = HashMap<SearchNode, Vec<SearchNode>>;

fn resolve_path_map<SearchNode>(
    distances: &Distances<SearchNode>,
    targets: &[SearchNode]
) -> PathMap<SearchNode> where
    SearchNode: Clone + PartialEq + PartialOrd + Ord + Hash,
{
    let mut queue = VecDeque::from_iter(targets.iter().cloned());
    let mut seen = HashSet::new();
    let mut path_map = HashMap::from_iter(
        targets.iter().map(|target| (target.clone(), vec![]))
    );

    while let Some(current) = queue.pop_front() {
        if !seen.insert(current.clone()) {
            continue;
        }

        for previous in &distances
            .get(&current)
            .map(|(_, previous)| previous.clone())
            .unwrap_or(vec![])
        {
            path_map.entry(previous.clone()).or_default().push(current.clone());
            queue.push_back(previous.clone());
        }
    }

    path_map
}

pub fn shortest_paths_to_target<SearchNode, IsTargetFn> (
    source: SearchNode,
    neighbors: impl Fn(&SearchNode) -> Vec<(SearchNode, usize)>,
    is_target: IsTargetFn,
) -> Option<(usize, PathMap<SearchNode>)> where
    SearchNode: Clone + PartialEq + PartialOrd + Ord + Hash,
    IsTargetFn: Fn(&SearchNode) -> bool + Copy,
{
    let (distances, reached_targets) = distances_impl(
        source,
        neighbors,
        is_target,
        true
    );

    if reached_targets.is_empty() {
        return None;
    }

    let potential_targets = reached_targets
        .iter()
        .filter_map(|target| Some((target, distances.get(target)?)))
        .collect_vec();

    let min_distance = potential_targets
        .iter()
        .map(|(_, (distance, _))| *distance)
        .min()?;

    let targets = potential_targets
        .iter()
        .filter(|(_, (distance, _))| *distance == min_distance)
        .map(|&(node, _)| node.clone())
        .collect_vec();

    Some((min_distance, resolve_path_map(&distances, &targets)))
}

pub fn distance_to_target<SearchNode, IsTargetFn> (
    source: SearchNode,
    neighbors: impl Fn(&SearchNode) -> Vec<(SearchNode, usize)>,
    is_target: IsTargetFn,
) -> Option<usize> where
    SearchNode: Copy + Clone + PartialEq + PartialOrd + Ord + Hash,
    IsTargetFn: Fn(&SearchNode) -> bool + Copy,
{
    let (distances, targets_reached) = distances_impl(
        source,
        neighbors,
        is_target,
        false,
    );

    if targets_reached.is_empty() {
        return None;
    }

    targets_reached
        .iter()
        .filter_map(|target| distances.get(target))
        .map(|(a, _)| *a)
        .min()
}

fn distances_impl<SearchNode, IsTargetFn> (
    source: SearchNode,
    neighbors: impl Fn(&SearchNode) -> Vec<(SearchNode, usize)>,
    is_target: IsTargetFn,
    break_at_target: bool,
) -> (Distances<SearchNode>, Vec<SearchNode>) where
    SearchNode: Clone + PartialEq + PartialOrd + Ord + Hash,
    IsTargetFn: Fn(&SearchNode) -> bool + Copy,
{
    let mut distances = HashMap::from([(source.clone(), (0, vec![]))]);
    let mut queue = BinaryHeap::from([(Reverse(0), source.clone())]);
    let mut targets_reached = vec![];

    while let Some((Reverse(distance), current)) = queue.pop() {
        if is_target(&current) {
            targets_reached.push(current.clone());

            if break_at_target {
                break;
            }
        }

        for (neighbor, cost) in neighbors(&current) {
            let (neighbor_distance, neighbor_sources) = distances
                .entry(neighbor.clone())
                .or_insert((usize::MAX, vec![]));

            match (distance + cost).cmp(neighbor_distance) {
                Ordering::Less => {
                    *neighbor_distance = distance + cost;
                    *neighbor_sources = vec![current.clone()];
                    queue.push((Reverse(*neighbor_distance), neighbor.clone()));
                }
                Ordering::Equal => {
                    neighbor_sources.push(current.clone());
                }
                Ordering::Greater => {},
            }
        }
    }

    (distances, targets_reached)
}
