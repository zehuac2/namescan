use std::io;

use clap::Parser;
use namescan::cli::Cli;
use namescan::io::OsFileSystem;
use namescan::report::StdioReporter;
use namescan::scan::DirectoryScanner;

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let reporter = StdioReporter::new(cli.report_increment);
    let file_system = OsFileSystem::new();
    let mut scanner = DirectoryScanner::new(file_system, reporter);

    scanner.scan(&cli.path)
}
