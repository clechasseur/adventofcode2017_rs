# adventofcode2017_rs

Solutions to the Advent of Code 2017 puzzles in Rust 🦀

Some fine folks on the [Exercism.org Discord server](https://exercism.org/r/discord) wanted to do the [Advent of Code 2017](https://adventofcode.com/2017) puzzles together.
I have already completed those in [Kotlin before](https://github.com/clechasseur/adventofcode2017), but since it's been a while, I figured I could do them again in Rust.

## Requirements

* [Rust](https://www.rust-lang.org/) 1.73.0 or later

## Running the tests

### All puzzles for each day

```sh
cargo test
```

#### With slow tests

```sh
cargo test --features slow
```

### Both puzzles for one day

```sh
cargo test day_01 --all-features
```

### Single puzzle

```sh
cargo test day_01_part_1 --all-features
```
