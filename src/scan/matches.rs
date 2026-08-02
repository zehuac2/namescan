use std::borrow::Cow;

use super::char_match::CharMatch;
use super::filename_scanner::FilenameScanner;
use super::os_set::OsSet;

/// An iterator that gives the forbidden characters in a file name.
///
/// The iterator makes no allocation for a name that is valid UTF-8, because
/// `OsStr::to_string_lossy` borrows such a name.
pub struct Matches<'a> {
    /// The file name.
    text: Cow<'a, str>,
    /// The byte offset of the next character in `text`.
    offset: usize,
    /// The index of the next character. [`CharMatch::index`] counts the
    /// characters and does not count the bytes.
    index: usize,
    /// The group of the operating system with the rule.
    os: OsSet,
}

impl<'a> Matches<'a> {
    pub(super) fn new(text: impl Into<Cow<'a, str>>, os: OsSet) -> Self {
        Self {
            text: text.into(),
            offset: 0,
            index: 0,
            os,
        }
    }
}

impl Iterator for Matches<'_> {
    type Item = CharMatch;

    fn next(&mut self) -> Option<CharMatch> {
        // An empty group has no rule. Each group contains the empty group,
        // thus the test below would give each character.
        if self.os.is_empty() {
            return None;
        }

        // The type keeps a byte offset and does not keep a `Chars`. A
        // `Chars` borrows the text, and this type owns the text.
        while let Some(character) = self.text[self.offset..].chars().next() {
            let index = self.index;
            self.offset += character.len_utf8();
            self.index += 1;

            if character.is_ascii() && FilenameScanner::TABLE[character as usize].contains(self.os) {
                return Some(CharMatch { character, index });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_backslash_is_forbidden_on_windows() {
        // `Path::file_name` treats '\\' as a separator on Windows, so a
        // `PathBuf` cannot carry a file name that contains it. Test the
        // character-detection logic directly on the raw name instead.
        let matches: Vec<_> = Matches::new("\\test.txt", OsSet::WINDOWS).collect();
        assert_eq!(
            matches,
            vec![CharMatch {
                character: '\\',
                index: 0
            }]
        );
    }

    #[test]
    fn index_counts_the_characters_and_not_the_bytes() {
        // The character 'é' has two bytes. The index of ':' is 1 and not 2.
        let matches: Vec<_> = Matches::new("é:x", OsSet::WINDOWS).collect();
        assert_eq!(
            matches,
            vec![CharMatch {
                character: ':',
                index: 1
            }]
        );
    }
}
