use std::io;
use std::path::Path;

use super::dir_entry::DirEntry;
use super::file_system::FileSystem;

/// A [`FileSystem`] that uses the real file system of the operating system.
#[derive(Debug, Default, Clone, Copy)]
pub struct OsFileSystem;

impl OsFileSystem {
    pub fn new() -> Self {
        Self
    }
}

impl FileSystem for OsFileSystem {
    fn list_dir(&self, path: &Path) -> io::Result<Vec<DirEntry>> {
        // Do not test the type of `path` before the read. A test with
        // `Path::is_dir` does one more `stat` system call for each item in
        // the tree. Let the read fail instead. Only the root of the scan
        // can be a file, because `DirEntry::is_dir` selects the other
        // directories.
        let entries = match std::fs::read_dir(path) {
            Ok(entries) => entries,
            Err(error) if error.kind() == io::ErrorKind::NotADirectory => {
                return Ok(Vec::new());
            }
            Err(error) => return Err(error),
        };

        let mut children = Vec::new();
        for entry in entries {
            let entry = entry?;
            // The operating system supplies the type of each child together
            // with the directory data. Thus `file_type` does no system call.
            // A test with `Path::is_dir` does one `stat` system call for
            // each child and makes the scan approximately 4 times slower.
            //
            // `file_type` does not follow a symbolic link. Thus the scan
            // does not go into a directory that a symbolic link points to.
            // This also prevents an endless loop when symbolic links make a
            // cycle.
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            children.push(DirEntry {
                path: entry.path(),
                is_dir,
            });
        }
        Ok(children)
    }
}
