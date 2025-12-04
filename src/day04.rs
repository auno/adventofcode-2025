use anyhow::{bail, Result, Error};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::grid::Grid;

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

#[aoc(day4, part1)]
fn part1(grid: &Input) -> usize {
    grid.into_iter()
        .filter(|(_, tile)| **tile == Tile::RollOfPaper)
        .filter(|(position, _)| {
            let count_neighboring_rolls = position.neighboring_positions()
                .into_iter()
                .filter(|p| grid.get(p).copied() == Some(Tile::RollOfPaper))
                .count();

            count_neighboring_rolls < 4
        })
        .count()
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
}
