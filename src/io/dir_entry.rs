use std::path::PathBuf;

/// One immediate child of a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirEntry {
    /// The full path of the child.
    pub path: PathBuf,
    /// True when the child is a directory.
    pub is_dir: bool,
}
