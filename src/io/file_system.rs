use std::io;
use std::path::Path;

use super::dir_entry::DirEntry;

/// A trait that gives access to the file system. Use a mock implementation
/// in tests.
pub trait FileSystem {
    /// Lists the immediate children of `path`.
    ///
    /// The list is empty when `path` is not a directory.
    fn list_dir(&self, path: &Path) -> io::Result<Vec<DirEntry>>;
}
