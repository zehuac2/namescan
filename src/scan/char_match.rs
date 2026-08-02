use std::fmt;

/// A forbidden character that the program finds in a file name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharMatch {
    /// The forbidden character.
    pub character: char,
    /// The index of the character in the file name.
    pub index: usize,
}

impl fmt::Display for CharMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.character)
    }
}
