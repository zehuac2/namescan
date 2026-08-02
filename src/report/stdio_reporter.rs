use std::io::{self, Write};

use crate::scan::ScanResult;
use crate::term::{CARRIAGE_RETURN, CLEAR_TO_LINE_END};

use super::reporter::Reporter;

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
        if let ScanResult::Invalid { path, os } = result {
            if self.is_last_line_progress {
                print!("{CARRIAGE_RETURN}{CLEAR_TO_LINE_END}");
                self.is_last_line_progress = false;
            }

            // Lock the standard output one time and write each character
            // directly. A `Vec` of `String`s and a `join` made more
            // allocations.
            let stdout = io::stdout();
            let mut out = stdout.lock();
            let _ = write!(out, "Invalid {}: {}; ", os, path.display());

            for (position, character_match) in result.matches().enumerate() {
                if position > 0 {
                    let _ = write!(out, ", ");
                }
                let _ = write!(out, "{}", character_match.character);
            }

            let _ = writeln!(out, " forbidden.");
        }
    }

    fn finish_file(&mut self) {
        self.count += 1;

        if self.count.is_multiple_of(self.report_increment) {
            print!("{CARRIAGE_RETURN}{} items scanned", self.count);
            let _ = io::stdout().flush();
            self.is_last_line_progress = true;
        }
    }

    fn finish(&mut self) {
        println!();
        println!("Finished. {} items scanned", self.count);
    }
}
