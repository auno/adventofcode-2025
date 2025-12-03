use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Joltage = usize;
type Bank = Vec<Joltage>;
type Input = Vec<Bank>;

fn parse_bank(bank: &str) -> Result<Bank> {
    (0..(bank.len()))
        .map(|i| &bank[i..(i + 1)])
        .map(|digit| digit.parse().with_context(|| "Invalid battery joltage: {digit} (in bank {line})"))
        .try_collect()
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(parse_bank)
        .try_collect()
}

fn largest_joltage(bank: &[Joltage], num_batteries: usize) -> Option<Joltage> {
    fn largest_joltage_with_cache(
        cache: &mut Vec<Vec<Option<Option<usize>>>>,
        bank: &[Joltage],
        bank_offset: usize,
        num_batteries: usize
    ) -> Option<usize> {
        if let Some(joltage) = cache[bank_offset][num_batteries] {
            return joltage;
        }

        let joltage =
            if (bank.len() - bank_offset) < num_batteries {
                None
            } else if num_batteries == 1 {
                bank[bank_offset..].iter().max().copied()
            } else {
                (bank_offset..bank.len())
                    .filter_map(|battery| {
                        let battery_joltage = bank[battery];
                        let subsequent_joltage = largest_joltage_with_cache(cache, bank, battery + 1, num_batteries - 1)?;
                        Some(battery_joltage * 10usize.pow((num_batteries - 1) as u32) + subsequent_joltage)
                    })
                    .max()
            };

        cache[bank_offset][num_batteries] = Some(joltage);
        joltage
    }

    let mut cache = vec![vec![None; 13]; bank.len() + 1];
    largest_joltage_with_cache(&mut cache, bank, 0, num_batteries)
}

#[aoc(day3, part1)]
fn part1(input: &Input) -> Joltage {
    input
        .iter()
        .filter_map(|bank| largest_joltage(bank, 2))
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Input) -> Joltage {
    input
        .iter()
        .filter_map(|bank| largest_joltage(bank, 12))
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE1: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn largest_joltage_example1_bank1_2() {
        assert_eq!(Some(98), largest_joltage(&parse_bank("987654321111111").unwrap(), 2));
    }

    #[test]
    fn largest_joltage_example1_bank2_2() {
        assert_eq!(Some(89), largest_joltage(&parse_bank("811111111111119").unwrap(), 2));
    }

    #[test]
    fn largest_joltage_example1_bank3_2() {
        assert_eq!(Some(78), largest_joltage(&parse_bank("234234234234278").unwrap(), 2));
    }

    #[test]
    fn largest_joltage_example1_bank4_2() {
        assert_eq!(Some(92), largest_joltage(&parse_bank("818181911112111").unwrap(), 2));
    }

    #[test]
    fn largest_joltage_example1_bank1_12() {
        assert_eq!(Some(987654321111), largest_joltage(&parse_bank("987654321111111").unwrap(), 12));
    }

    #[test]
    fn largest_joltage_example1_bank2_12() {
        assert_eq!(Some(811111111119), largest_joltage(&parse_bank("811111111111119").unwrap(), 12));
    }

    #[test]
    fn largest_joltage_example1_bank3_12() {
        assert_eq!(Some(434234234278), largest_joltage(&parse_bank("234234234234278").unwrap(), 12));
    }

    #[test]
    fn largest_joltage_example1_bank4_12() {
        assert_eq!(Some(888911112111), largest_joltage(&parse_bank("818181911112111").unwrap(), 12));
    }

    #[test]
    fn part1_example1() {
        assert_eq!(357, part1(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(17278, part1(&parse(include_str!("../input/2025/day3.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(3121910778619, part2(&parse(EXAMPLE1).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(171528556468625, part2(&parse(include_str!("../input/2025/day3.txt")).unwrap()));
    }
}
