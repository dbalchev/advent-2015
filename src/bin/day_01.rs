use std::io::BufRead;

use advent_2015::aoc::{solution::Solution, utils::AnyResult};

#[derive(PartialEq, Eq, Debug)]
struct Day01 {
    instructions: String,
}

static FLOOR_UPDATE_TABLE: [i64; 256] = {
    let mut table = [0; 256];
    table['(' as usize] = 1;
    table[')' as usize] = -1;
    table
};

impl Solution for Day01 {
    const DAY_NUMBER: i32 = 1;
    type SolutionResult = i64;

    fn parse_input<B: BufRead>(reader: B) -> AnyResult<Self> {
        let instructions = reader
            .lines()
            .next()
            .ok_or_else(|| "No first line".to_string())??;
        Ok(Self { instructions })
    }

    fn solve_part_1(&self) -> AnyResult<Self::SolutionResult> {
        Ok(self
            .instructions
            .as_bytes()
            .iter()
            .map(|&b| FLOOR_UPDATE_TABLE[b as usize])
            .sum())
    }

    fn solve_part_2(&self) -> AnyResult<Self::SolutionResult> {
        Ok(self
            .instructions
            .as_bytes()
            .iter()
            .map(|&b| FLOOR_UPDATE_TABLE[b as usize])
            .scan(0, |st, item| {
                *st += item;
                if *st < 0 {
                    None
                } else {
                    Some(*st)
                }
            })
            .count() as i64
            + 1)
    }
}

fn main() -> AnyResult<()> {
    Day01::day_main()
}

#[cfg(test)]
mod day_01_tests {
    use advent_2015::aoc::solution::Solution;

    use super::Day01;
    fn example_data() -> Day01 {
        Day01 {
            instructions: "(())".to_string(),
        }
    }
    #[test]
    fn test_part_1() {
        assert_eq!(0, example_data().solve_part_1().unwrap());
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            5,
            Day01 {
                instructions: "()())".to_string()
            }
            .solve_part_2()
            .unwrap()
        );
    }
}
