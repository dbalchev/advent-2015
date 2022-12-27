# About

This is a template for easier implementations of [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

Some of the features of thist template include:

* The `Solution` trait that abstracts some of the boilerplate for each task.
* Utilities to automatically download the puzzle inputs.
* A copy and modify flow for quick day setup.

# General setup

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this Repo
3. (Optional) go to `src/aoc/data.rs` and change the year

# Running day_00

I've used day_00 as a test day and show examples of how to use the flow.

## Running the whole task

```bash
cargo run --bin day_00 -- <options>
```

This command builds and runs day_00 on input dependent on `<options>`. 
When day_00 is ran the input is parsed and the answers for both parts are shown.


- if `<options>` are empty input is read from stdin
- if `--file <file>` is passed input is read from the `<file>`
- if `--example` is passed, the example input for the corresponsing day is downloaded, saved in `.cache/` and passed as input.
- if `--real` is passed, the puzzle input for hte corresponding day is downloaded, saved in `.cache/` and passed as input. 
  If a user session isn't found in `.cache` you are prompted to provide it and after that it's saved.

```bash
cargo test day_00_tests
```

This command runs the tests in the `day_00_tests` module. You can also use `cargo test` to run all tests.

# Day setup

1. Copy `src/bin/day_00.rs` to `src/bin/day_##.rs`.
2. Update the name of the struct and all mentions in your new file.
3. Change `DAY_NUMBER` and possibly `SolutionResult` if the output isn't an integer.
