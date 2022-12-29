use clap::Parser;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{stdin, BufRead, BufReader, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use super::data::{get_example_data, get_real_data};
use super::utils::AnyResult;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, group = "input")]
    example: bool,
    #[arg(short, long, group = "input")]
    real: bool,
    #[arg(short, long, group = "input")]
    file: Option<String>,
}

fn get_input_reader(cli: Cli, day: i32) -> AnyResult<Box<dyn BufRead>> {
    if cli.example {
        let file = get_example_data(day)?;
        Ok(Box::new(BufReader::new(file)))
    } else if cli.real {
        let file = get_real_data(day)?;
        Ok(Box::new(BufReader::new(file)))
    } else if let Some(file_path) = cli.file {
        let file = OpenOptions::new().read(true).write(false).open(file_path)?;
        Ok(Box::new(BufReader::new(file)))
    } else {
        Ok(Box::new(stdin().lock()))
    }
}

pub trait Solution: Sized {
    const DAY_NUMBER: i32;
    type SolutionResult: Display;
    fn parse_input<B: BufRead>(reader: B) -> AnyResult<Self>;
    fn solve_part_1(&self) -> AnyResult<Self::SolutionResult>;
    fn solve_part_2(&self) -> AnyResult<Self::SolutionResult>;

    fn example_data_reader() -> AnyResult<Box<dyn BufRead>> {
        let file = get_example_data(Self::DAY_NUMBER)?;
        Ok(Box::new(BufReader::new(file)))
    }

    fn day_main() -> AnyResult<()> {
        let reader = get_input_reader(Cli::parse(), Self::DAY_NUMBER)?;
        let solution = Self::parse_input(reader)?;
        let part_1_answer = solution.solve_part_1()?;
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(&mut stdout, "Part 1: ")?;
        stdout.set_color(ColorSpec::new().set_fg(None))?;
        writeln!(&mut stdout, "{}", part_1_answer)?;
        let part_2_answer = solution.solve_part_2()?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(&mut stdout, "Part 2: ")?;
        stdout.set_color(ColorSpec::new().set_fg(None))?;
        writeln!(&mut stdout, "{}", part_2_answer)?;
        Ok(())
    }
}
