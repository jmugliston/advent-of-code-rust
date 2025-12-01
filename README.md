🎄 Advent of Code (Rust) 🎄

<div align="left">
    <img
      src="https://github.com/jmugliston/advent-of-code-rust/raw/HEAD/aoc-rust.png"
      width="200"
      height="auto"
    />
</div>

Project for AOC challenges and solutions in Rust.

## Setup

Create a .env file with the following:

```
SESSION_TOKEN=<aoc-session-token>
```

## Install

```sh
cargo install
```

## Test

```sh
cargo test
```

### Usage

Initialise a day:

```sh
cargo run init --year=2025 --day=1
```

Run the solution for a day:

```sh
cargo run solve --year=2024 --day=1 --part=1
```
