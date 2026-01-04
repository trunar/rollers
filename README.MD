# rolle*rs*

A Rust 🦀 CLI dice roller.

[![Rust](https://github.com/trunar/rollers/actions/workflows/rust.yml/badge.svg)](https://github.com/trunar/rollers/actions/workflows/rust.yml)

## Prerequisites

If you want to use Cargo for installation or to build the binaries yourself, ensure you have the Rust toolchain installed. If you don't have it yet, you can get it via rustup.rs.

## Installation

### a. From Releases page

Temporary unavailable.

### b. From crates.io using Cargo

```sh
cargo install rollers
```

### c. Directly from GitHub using Cargo

```sh
cargo install --git https://github.com/trunar/rollers
```

## Build yourself

If you are not content with installation options, you could build the binaries yourself.
Follow these commands to clone the repository and compile the binary:

```sh
# Clone the repository
git clone https://github.com/trunar/rollers
cd rollers

# Build the project in release mode
cargo build --release
```

Once the build is complete, you can find the executable in the target/release directory. Use the -h flag to view the help menu:

```sh
./target/release/rollers -h
```

## Usage

```sh
Usage: rollers [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Dice notation (e.g., 2d6, 4dF, 1d20+5)

Options:
  -q, --quiet        Only show the final result
  -a, --average      Show the average instead of rolling
      --highest <N>  Keep only the highest N dice
      --lowest <N>   Keep only the lowest N dice
  -h, --help         Print help
  -V, --version      Print version
```

## Examples

```sh
$ rollers 3d6+1

  Pool:      6, 6, 6
  Modifier:  +1
  Total:     19
```

```sh
$ rollers 4dF

  Pool:      -, +, +, 0
  Total:     1
```

```sh
$ rollers 1d20 --quiet
3
```

```sh
$ rollers 2d6 --average

  Average: 7.00
```

```sh
$ rollers 3d20 --highest 2

  Pool:      4, 7, 15
  Kept:      15, 7
  Total:     22
```

## To-Do

- Drop Logic: Add `--drop-highest N` or `--drop-lowest N` (useful for "rolling for stats" where you roll 4d6 and drop the lowest 1).
- Exploding Dice: Add a `-e` flag where rolling the maximum value on a die allows you to roll it again and add it to the total.
- Multiple Arguments: Allow rolling things like `rollers 1d20+5 2d6` in one go.
