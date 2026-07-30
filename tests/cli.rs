use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use namescan::cli::Cli;

#[test]
fn command_definition_is_valid() {
    Cli::command().debug_assert();
}

#[test]
fn parses_defaults() {
    let cli = Cli::try_parse_from(["namescan"]).unwrap();
    assert_eq!(cli.path, PathBuf::from("."));
    assert_eq!(cli.report_increment, 100);
}

#[test]
fn parses_arguments() {
    let cli = Cli::try_parse_from(["namescan", "/tmp", "--report-increment", "10"]).unwrap();
    assert_eq!(cli.path, PathBuf::from("/tmp"));
    assert_eq!(cli.report_increment, 10);

    let cli = Cli::try_parse_from(["namescan", "-r", "5"]).unwrap();
    assert_eq!(cli.report_increment, 5);
}
