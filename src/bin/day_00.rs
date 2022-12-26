use std::io::BufRead;

use aoc_template_rs::aoc::{solution::Solution, utils::AnyResult};

#[derive(PartialEq, Eq, Debug)]
struct Day00 {
    lines: Vec<String>,
}

impl Solution for Day00 {
    const DAY_NUMBER: i32 = 1;
    type SolutionResult = i32;

    fn parse_input(reader: Box<dyn BufRead>) -> AnyResult<Self> {
        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        Ok(Self { lines })
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

#[cfg(test)]
mod day00_tests {
    use aoc_template_rs::aoc::solution::Solution;

    use super::Day00;
    fn example_data() -> Day00 {
        Day00 {
            lines: vec![
                "1000".to_string(),
                "2000".to_string(),
                "3000".to_string(),
                "".to_string(),
                "4000".to_string(),
                "".to_string(),
                "5000".to_string(),
                "6000".to_string(),
                "".to_string(),
                "7000".to_string(),
                "8000".to_string(),
                "9000".to_string(),
                "".to_string(),
                "10000".to_string(),
            ],
        }
    }
    #[test]
    fn test_parsing() {
        assert_eq!(
            example_data(),
            Day00::parse_input(Day00::example_data_reader().unwrap()).unwrap()
        );
    }
    #[test]
    fn test_part_1() {
        assert_eq!(14, example_data().solve_part_1().unwrap());
    }
    #[test]
    fn test_part_2() {
        assert_eq!(41, example_data().solve_part_2().unwrap());
    }
}
