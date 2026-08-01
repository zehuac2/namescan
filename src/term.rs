//! ANSI escape sequences for terminal output.

/// Move the cursor to the start of the current line.
pub const CARRIAGE_RETURN: &str = "\r";
/// Erase from the cursor to the end of the current line.
pub const CLEAR_TO_LINE_END: &str = "\x1B[K";
