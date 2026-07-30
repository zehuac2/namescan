use std::path::PathBuf;

use clap::Parser;

/// The program finds the characters that Windows and macOS do not permit in
/// file names.
#[derive(Parser, Debug)]
#[command(name = "namescan")]
pub struct Cli {
    /// The root directory that the program scans
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// The number of scanned items between two progress reports
    #[arg(short, long, default_value_t = 100)]
    pub report_increment: usize,
}
