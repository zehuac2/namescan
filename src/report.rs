use std::io::{self, Write};

use crate::scan::ScanResult;

/// A reporter that receives the [`ScanResult`]s of a directory scan.
pub trait Reporter {
    /// Reports the scan result of one file name.
    fn report(&mut self, result: &ScanResult<'_>);
    /// Ends the scan of one file or directory.
    fn finish_file(&mut self);
    /// Ends the full scan.
    fn finish(&mut self);
}

/// A [`Reporter`] that prints invalid results and the scan progress to the
/// standard output.
pub struct StdioReporter {
    /// The number of scanned items between two progress reports.
    pub report_increment: usize,
    count: usize,
    is_last_line_progress: bool,
}

impl StdioReporter {
    pub fn new(report_increment: usize) -> Self {
        Self {
            report_increment: report_increment.max(1),
            count: 0,
            is_last_line_progress: false,
        }
    }
}

impl Reporter for StdioReporter {
    fn report(&mut self, result: &ScanResult<'_>) {
        if let ScanResult::Invalid { path, matches, os } = result {
            if self.is_last_line_progress {
                print!("\r");
                self.is_last_line_progress = false;
            }
            let characters = matches
                .iter()
                .map(|m| m.character.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            println!("Invalid {}: {}; {} forbidden.", os, path.display(), characters);
        }
    }

    fn finish_file(&mut self) {
        self.count += 1;

        if self.count.is_multiple_of(self.report_increment) {
            print!("\r{} items scanned", self.count);
            let _ = io::stdout().flush();
            self.is_last_line_progress = true;
        }
    }

    fn finish(&mut self) {
        println!();
        println!("Finished. {} items scanned", self.count);
    }
}
