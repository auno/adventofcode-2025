use anyhow::{bail, Result, Error};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::grid::{Grid, Position};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Floor,
    RollOfPaper,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Tile::Floor),
            '@' => Ok(Tile::RollOfPaper),
            _ => bail!("Invalid Tile: {value}"),
        }
    }
}

type Input = Grid<Tile>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Result<Input> {
    Grid::parse(input)
}

fn list_removable_rolls(grid: &Input) -> impl IntoIterator<Item = Position> {
    grid.into_iter()
        .filter(|(_, tile)| **tile == Tile::RollOfPaper)
        .filter(|(position, _)| {
            let count_neighboring_rolls = position.neighboring_positions()
                .into_iter()
                .filter(|p| grid.get(p).copied() == Some(Tile::RollOfPaper))
                .count();

            count_neighboring_rolls < 4
        })
        .map(|(p, _)| p)
}

#[aoc(day4, part1)]
fn part1(grid: &Input) -> usize {
    list_removable_rolls(grid)
        .into_iter()
        .count()
}

#[aoc(day4, part2)]
fn part2(grid: &Input) -> usize {
    let mut grid = grid.clone();
    let mut count = 0;

    loop {
        let removable = list_removable_rolls(&grid).into_iter().collect_vec();

        if removable.is_empty() {
            break;
        }

        count += removable.len();

        for p in removable {
            grid.set(&p, Tile::Floor);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(13, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1578, part1(&parse(include_str!("../input/2025/day4.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(43, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(10132, part2(&parse(include_str!("../input/2025/day4.txt")).unwrap()));
    }
}
