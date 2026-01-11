# rolle*rs*

A Rust 🦀 CLI dice roller.

[![Rust](https://github.com/trunar/rollers/actions/workflows/rust.yml/badge.svg)](https://github.com/trunar/rollers/actions/workflows/rust.yml)

## Prerequisites

If you want to use Cargo for installation or to build the binaries yourself, ensure you have the Rust toolchain installed. If you don't have it yet, you can get it via [rustup.rs](https://rustup.rs/).

## Installation

### a. From Releases page

1. Download the archive for your operating system from the [Latest Release](https://github.com/trunar/rollers/releases/latest).
2. Follow the steps for your system below:

#### Linux (x86_64)

The `musl` build is a static binary that works on almost any distribution.

* Extract: `tar -xzvf rollers-x86_64-unknown-linux-musl.tar.gz`
* Install: `sudo mv rollers /usr/local/bin/`
* Permissions: `sudo chmod +x /usr/local/bin/rollers`

#### macOS (Apple Silicon or Intel)

* Extract: `tar -xzvf rollers-apple-darwin.tar.gz`
* Install: `sudo mv rollers /usr/local/bin/`
* Security: If macOS blocks the binary from running, use:
  `xattr -d com.apple.quarantine /usr/local/bin/rollers`

#### Windows (x86_64)

* Extract: Right-click the `.zip` file and select "Extract All".
* Install: Move `rollers.exe` to a folder in your System PATH (e.g., `C:\Windows\`).
* Note: If Windows SmartScreen warns you, click "More Info" and "Run anyway".

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

---

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

**Standard roll (with modifier):**

```sh
$ rollers 3d6+1

  Pool:      6, 6, 6
  Modifier:  +1
  Total:     19
```

**FUDGE / Fate roll:**

```sh
$ rollers 4dF

  Pool:      -, +, +, 0
  Total:     1
```

**Quiet mode (script friendly):**

```sh
$ rollers 1d20 --quiet
3
```

**Average of a roll:**

```sh
$ rollers 2d6 --average

  Average: 7.00
```

**Keep 2 highest dice:**

```sh
$ rollers 3d20 --highest 2

  Pool:      15, 7, 4
  Kept:      15, 7
  Total:     22
```

**Drop 1 lowest die:**

```sh
rollers 4d6 --drop-lowest 1

  Pool:      6, 5, 2, 2
  Kept:      6, 5, 2
  Total:     13
```

---

## To-Do

* ✅~~Drop Logic: Add `--drop-highest N` or `--drop-lowest N` (useful for "rolling for stats" where you roll 4d6 and drop the lowest 1).~~
* Exploding Dice: Add a `-e` flag where rolling the maximum value on a die allows you to roll it again and add it to the total.
* Multiple Arguments: Allow rolling things like `rollers 1d20+5 2d6` in one go.
