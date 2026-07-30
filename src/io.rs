use std::io;
use std::path::{Path, PathBuf};

/// A trait that gives access to the file system. Use a mock implementation
/// in tests.
pub trait FileSystem {
    /// Lists the immediate children of `path`.
    ///
    /// The list is empty when `path` is not a directory.
    fn list_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>>;
}

/// A [`FileSystem`] that uses the real file system of the operating system.
#[derive(Debug, Default, Clone, Copy)]
pub struct OsFileSystem;

impl OsFileSystem {
    pub fn new() -> Self {
        Self
    }
}

impl FileSystem for OsFileSystem {
    fn list_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        if !path.is_dir() {
            return Ok(Vec::new());
        }

        let mut children = Vec::new();
        for entry in std::fs::read_dir(path)? {
            children.push(entry?.path());
        }
        Ok(children)
    }
}
