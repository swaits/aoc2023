# Advent of Code 2023 Solutions

## Overview

This repository contains my personal solutions to the Advent of Code 2023
challenges, a series of programming puzzles that cover a variety of skill sets
and problem-solving techniques.

## Building and Running

This project is developed in Rust, and you can build, test, and run it like any
standard Rust project using Cargo.

```bash
git clone https://git.sr.ht/~swaits/aoc2023
cd aoc2023
cargo run -- ARGUMENT
```

Where `ARGUMENT` can be:

- A specific day number (e.g., 1, 2, 25) to run the solution for that day's challenge.
- all to execute all the implemented solutions sequentially.

Running ...

```cash
cargo test
```

... will run all the tests for all the days, which include both the example
inputs and outputs provided in each day's problem description as well as my
personal user-specific inputs and accepted outputs.

## Implementation

Each dayXX.rs file within the project represents the solution to the problem
for that specific day of the Advent of Code.

### Error Handling

The program is designed to panic in case of errors. However, such scenarios are
unlikely as tests cover both the example inputs/outputs provided in the problem
descriptions and my user-specific data inputs/outputs.

## License

This project is licensed under the MIT License.

## Contact

For any queries regarding this project, feel free to contact me at
[steve@waits.net](mailto:steve@waits.net).
