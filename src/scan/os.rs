use std::fmt;

/// An operating system with a file name rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Os {
    Windows,
    Linux,
    MacOs,
}

impl fmt::Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Os::Windows => write!(f, "windows"),
            Os::Linux => write!(f, "linux"),
            Os::MacOs => write!(f, "macOS"),
        }
    }
}
