//! The file system access.
//!
//! One file holds one type. This file only declares the submodules and
//! re-exports their public types.

mod dir_entry;
mod file_system;
mod os_file_system;

pub use dir_entry::DirEntry;
pub use file_system::FileSystem;
pub use os_file_system::OsFileSystem;
