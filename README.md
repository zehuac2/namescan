# `namescan`

> File name scanner to detect file names that cannot be synced between OS

## Installation

```sh
# with mise
mise use -g github:zehuac2/namescan

# with brew
brew tap zehuac2/tools
brew install zehuac2/tools/namescan
```

## Usage

```sh
namescan [PATH] [-r|--report-increment <N>]
```

- `PATH` — root directory to scan (default: `.`)
- `-r`, `--report-increment` — number of items scanned between progress reports (default: `100`)

File names containing characters forbidden on Windows (`< > : " / \ | ? *`) or macOS (`:`) are reported.

## Build

```sh
cargo build --release
```

## Test

```sh
cargo test
```
