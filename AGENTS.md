# AGENTS.md

This file gives instructions to agents that work on this project. Obey all
the instructions in this file.

## Project description

namescan is a command-line program. The program scans the file names in a
directory tree. The program finds the characters that Windows and macOS do
not permit in file names.

## Development environment

- The program is written in Rust.
- mise manages the Rust toolchain.
- The only dependency is clap.

## Project structure

| Path           | Description                                 |
| -------------- | ------------------------------------------- |
| `src/main.rs`  | The entry point of the program.             |
| `src/cli.rs`   | The command-line interface.                 |
| `src/io.rs`    | The file system access.                     |
| `src/scan.rs`  | The scan logic and the scan result types.   |
| `src/report.rs`| The output of the scan results.             |
| `tests/cli.rs` | The tests for the command-line interface.   |

## Comment rules

- Write all comments and all documentation in Simplified Technical English
  (STE).
- Use a maximum of 20 words in one sentence.
- Use the active voice.
- Use the imperative mood when you give an instruction.
- Use the same word for the same thing. Do not use synonyms.
- Do not use slang or idioms.

## Code verification

Do this procedure after each change to the code:

1. Build the program: `cargo build`. Make sure that there are no errors.
2. Run the linter: `cargo clippy --all-targets`. Make sure that there are no
   warnings.
3. Run the tests: `cargo test`. Make sure that all the tests pass.
4. Run the program on a test directory. Make sure that the output is correct.

Do not commit changes to git unless the user tells you to do so.
