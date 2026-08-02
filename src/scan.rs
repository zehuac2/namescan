//! The scan logic and the scan result types.
//!
//! One file holds one type. This file only declares the submodules and
//! re-exports their public types.

mod char_match;
mod directory_scanner;
mod filename_scanner;
mod matches;
mod os;
mod os_set;
mod scan_result;
mod scan_results;

pub use char_match::CharMatch;
pub use directory_scanner::DirectoryScanner;
pub use filename_scanner::FilenameScanner;
pub use matches::Matches;
pub use os::Os;
pub use os_set::OsSet;
pub use scan_result::ScanResult;
pub use scan_results::ScanResults;
