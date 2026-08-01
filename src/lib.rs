//! Library core of `namescan`.
//!
//! The program scans the file names in a directory tree. The program finds
//! the forbidden characters in the file names.

pub mod cli;
pub mod io;
pub mod report;
pub mod scan;
pub mod term;
