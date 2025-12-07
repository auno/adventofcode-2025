use anyhow::{bail, Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::grid::{Direction, Grid, Position};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Space,
    Source,
    Splitter,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Space),
            'S' => Ok(Self::Source),
            '^' => Ok(Self::Splitter),
            _ => bail!("Invalid tile: {value}")
        }
    }
}

type Input = (Grid<Tile>, Position);

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Input> {
    let (grid, source_positions) = Grid::parse_with_position_detection(input, &['S'], None)?;
    let source_position = source_positions
        .get(&'S')
        .and_then(|ps| ps.first())
        .copied()
        .with_context(|| "Source position not found".to_string())?;

    Ok((grid, source_position))
}

#[aoc(day7, part1)]
fn part1((grid, source_position): &Input) -> usize {
    let mut beams = vec![source_position.1];
    let mut count_splits = 0;

    for i in (source_position.0 + 1)..grid.rows() {
        let mut next_beams = vec![];

        for &j in &beams {
            if grid.get(&Position(i, j)) == Some(&Tile::Splitter) {
                next_beams.extend([j - 1, j + 1]);
                count_splits += 1;
            } else {
                next_beams.push(j);
            }
        }

        beams = next_beams.into_iter().unique().collect_vec()
    }

    count_splits
}

fn count_timelines(position: Position, grid: &Grid<Tile>) -> usize {
    type Cache = Vec<Vec<Option<usize>>>;

    fn count_timelines_cached(cache: &mut Cache, position: Position, grid: &Grid<Tile>) -> usize {
        if let Some(count) = cache
            .get(position.0 as usize)
            .and_then(|rc| rc.get(position.1 as usize))
            .copied()
            .flatten()
        {
            return count;
        }

        let Some(tile) = grid.get(&position) else {
            return 1;
        };

        let count = match tile {
            Tile::Space => count_timelines_cached(cache, position.step(Direction::Down), grid),
            Tile::Source => count_timelines_cached(cache, position.step(Direction::Down), grid),
            Tile::Splitter => {
                count_timelines_cached(cache, position.step(Direction::Left), grid)
                    + count_timelines_cached(cache, position.step(Direction::Right), grid)
            }
        };

        cache[position.0 as usize][position.1 as usize] = Some(count);
        count
    }

    count_timelines_cached(&mut vec![vec![None; grid.cols()]; grid.rows()], position, grid)
}

#[aoc(day7, part2)]
fn part2((grid, source_position): &Input) -> usize {
    count_timelines(*source_position, grid)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn part1_example1() {
        assert_eq!(21, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1546, part1(&parse(include_str!("../input/2025/day7.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(40, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(13883459503480, part2(&parse(include_str!("../input/2025/day7.txt")).unwrap()));
    }
}
