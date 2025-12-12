use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{izip, Itertools};
use sscanf::sscanf;

type Input = ([[bool; 9]; 6], Vec<(usize, usize, [usize; 6])>);

#[aoc_generator(day12)]
fn parse(input: &str) -> Result<Input> {
    let (shapes, regions) = input
        .rsplit_once("\n\n")
        .context("Invalid input")?;

    let shapes = shapes
        .split("\n\n")
        .map(|shape| {
            shape
                .lines()
                .skip(1)
                .flat_map(|line| line.chars().map(|c| c == '#'))
                .collect_vec()
                .try_into()
                .or_else(|_| bail!("Invalid shape:\n{shape}"))
        })
        .collect::<Result<Vec<_>>>()?
        .try_into()
        .or_else(|_| bail!("Invalid shapes"));

    let regions = regions
        .lines()
        .map(|line| {
            sscanf!(line, "{usize}x{usize}: {usize} {usize} {usize} {usize} {usize} {usize}")
                .or_else(|_| bail!("Invalid region: {line}"))
                .map(|(width, height, c0, c1, c2, c3, c4, c5)| {
                    (width, height, [c0, c1, c2, c3, c4, c5])
                })
        })
        .try_collect();

    Ok((shapes?, regions?))
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    let (shapes, regions) = input;

    let shape_sizes = shapes
        .iter()
        .map(|shape| shape.iter().filter(|b| **b).count())
        .collect_vec();

    regions
        .iter()
        .filter(|(width, height, shape_counts)| {
            let min_area_needed = izip!(shape_counts, &shape_sizes)
                .map(|(shape_count, shape_size)| shape_size * shape_count)
                .sum::<usize>();
            let max_area_needed = shape_counts.iter().sum::<usize>() * 9;

            if min_area_needed > width * height {
                false
            } else if max_area_needed <= width * height {
                true
            } else {
                unimplemented!()
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[allow(dead_code)]
    const EXAMPLE1: &str = indoc! {"
        0:
        ###
        ##.
        ##.

        1:
        ###
        ##.
        .##

        2:
        .##
        ###
        ##.

        3:
        ##.
        ###
        ##.

        4:
        ###
        #..
        ###

        5:
        ###
        .#.
        ###

        4x4: 0 0 0 0 2 0
        12x5: 1 0 1 0 2 2
        12x5: 1 0 1 0 3 2
    "};

    // #[test]
    // fn part1_example1() {
    //     assert_eq!(2, part1(&parse(EXAMPLE1).unwrap()));
    // }

    #[test]
    fn part1_input() {
        assert_eq!(579, part1(&parse(include_str!("../input/2025/day12.txt")).unwrap()));
    }
}
