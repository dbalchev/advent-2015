use std::io::BufRead;

use aoc_template_rs::aoc::{solution::Solution, utils::AnyResult};

struct Day00 {
    lines: Vec<String>,
}

impl Solution for Day00 {
    const DAY_NUMBER: i32 = 1;
    type SolutionResult = i32;

    fn parse_input(reader: Box<dyn BufRead>) -> AnyResult<Self> {
        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        Ok(Day00 { lines })
    }

    fn solve_part_1(&self) -> AnyResult<Self::SolutionResult> {
        Ok(self.lines.len() as i32)
    }

    fn solve_part_2(&self) -> AnyResult<Self::SolutionResult> {
        Ok(self.lines.iter().map(|s| s.len()).sum::<usize>() as i32)
    }
}

fn main() -> AnyResult<()> {
    Day00::day_main()
}
