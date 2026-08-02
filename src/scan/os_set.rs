use super::os::Os;

/// A group of operating systems.
///
/// The type gives the operations of a set. It holds each member in one bit
/// of one byte. Thus one table entry can tell which operating systems forbid
/// one byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OsSet(u8);

impl OsSet {
    /// A group with no member.
    pub const EMPTY: Self = Self(0);
    /// A group with only Windows.
    pub const WINDOWS: Self = Self(1);
    /// A group with only macOS.
    pub const MACOS: Self = Self(1 << 1);

    /// Gives the group that holds only `os`.
    pub const fn of(os: Os) -> Self {
        match os {
            Os::Windows => Self::WINDOWS,
            Os::MacOs => Self::MACOS,
            // Linux forbids only '/'. A file name cannot contain '/'. Thus
            // the scan gives no result for Linux.
            Os::Linux => Self::EMPTY,
        }
    }

    /// Tells if the group has no member.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Tells if the group holds each member of `other`.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Gives the group with the members of the two groups.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_set_holds_the_operating_systems() {
        assert!(OsSet::EMPTY.is_empty());
        assert!(!OsSet::WINDOWS.is_empty());

        let both = OsSet::WINDOWS.union(OsSet::MACOS);
        assert!(both.contains(OsSet::WINDOWS));
        assert!(both.contains(OsSet::MACOS));
        assert!(!OsSet::WINDOWS.contains(OsSet::MACOS));

        assert_eq!(OsSet::of(Os::Windows), OsSet::WINDOWS);
        assert_eq!(OsSet::of(Os::MacOs), OsSet::MACOS);
        // Linux forbids only '/'. A file name cannot contain '/'.
        assert!(OsSet::of(Os::Linux).is_empty());
    }
}
