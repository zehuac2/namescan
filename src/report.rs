//! The output of the scan results.
//!
//! One file holds one type. This file only declares the submodules and
//! re-exports their public types.

mod reporter;
mod stdio_reporter;

pub use reporter::Reporter;
pub use stdio_reporter::StdioReporter;
