use std::fmt;
use std::path::Path;

use super::matches::Matches;
use super::os::Os;
use super::os_set::OsSet;

/// The result of the scan of a single file name.
///
/// The result borrows the path and does not own it. A `PathBuf` copies the
/// full path, thus an owned path made one allocation and one copy for each
/// item in the tree. The reporter reads the path and then the program
/// discards the result, thus the result does not need its own path.
///
/// The result also does not keep the forbidden characters. A `Vec` of the
/// characters made one allocation for each invalid name. [`Self::matches`]
/// finds the characters again.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanResult<'a> {
    Ok(&'a Path),
    Invalid { path: &'a Path, os: Os },
}

impl<'a> ScanResult<'a> {
    /// Gives the forbidden characters in the file name.
    ///
    /// The result does not keep the characters, thus this method reads the
    /// name one more time. Only the report needs the characters, and the
    /// report runs only for an invalid name. A correct name gives no
    /// character.
    pub fn matches(&self) -> Matches<'a> {
        match *self {
            ScanResult::Ok(_) => Matches::new("", OsSet::EMPTY),
            ScanResult::Invalid { path, os } => Matches::new(
                path.file_name().unwrap_or_default().to_string_lossy(),
                OsSet::of(os),
            ),
        }
    }
}

impl fmt::Display for ScanResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanResult::Ok(path) => write!(f, "OK: {}", path.display()),
            ScanResult::Invalid { path, os } => {
                write!(f, "Invalid: {}, Characters: ", path.display())?;
                // Write each character directly. A `Vec` of `String`s and a
                // `join` made more allocations.
                for (position, character_match) in self.matches().enumerate() {
                    if position > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", character_match.character)?;
                }
                write!(f, ", OS: {os}")
            }
        }
    }
}
