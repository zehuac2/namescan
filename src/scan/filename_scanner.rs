use std::path::Path;

use super::os::Os;
use super::os_set::OsSet;
use super::scan_result::ScanResult;
use super::scan_results::ScanResults;

/// A table that tells for each byte which operating systems forbid the byte.
type ForbiddenTable = [OsSet; 256];

/// Adds the forbidden characters of one operating system to `table`.
///
/// The table gives a test that takes a constant time. A search with
/// `slice::contains` compares the character with each character in the list.
/// One table for all the operating systems gives one pass over the name.
///
/// The function runs at compile time. It stops the build if a character is
/// not ASCII, because the table holds one entry for each byte value.
const fn add_to_table(mut table: ForbiddenTable, os: OsSet, forbidden: &[char]) -> ForbiddenTable {
    let mut index = 0;

    while index < forbidden.len() {
        let character = forbidden[index];
        assert!(character.is_ascii(), "a forbidden character must be ASCII");
        table[character as usize] = table[character as usize].union(os);
        index += 1;
    }

    table
}

/// The scanner that finds forbidden characters in file names.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilenameScanner;

impl FilenameScanner {
    /// Characters forbidden in Windows file names.
    pub const WINDOWS_FORBIDDEN: &'static [char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    /// Characters forbidden in macOS file names.
    pub const MACOS_FORBIDDEN: &'static [char] = &[':'];

    /// The table for all the operating systems.
    pub(super) const TABLE: ForbiddenTable = add_to_table(
        add_to_table(
            [OsSet::EMPTY; 256],
            OsSet::WINDOWS,
            Self::WINDOWS_FORBIDDEN,
        ),
        OsSet::MACOS,
        Self::MACOS_FORBIDDEN,
    );

    pub fn new() -> Self {
        Self
    }

    /// Scans the file name of `path`.
    ///
    /// The function returns one [`ScanResult::Invalid`] for each violated
    /// file name rule. The function returns one [`ScanResult::Ok`] when the
    /// file name has no forbidden characters.
    pub fn scan<'a>(&self, path: &'a Path) -> ScanResults<'a> {
        // Examine the raw bytes of the name. `OsStr::to_string_lossy` makes
        // a `String` for each file name, also for a correct name. The bytes
        // of an `OsStr` hold each ASCII character as itself on all the
        // platforms. All the forbidden characters are ASCII. Thus the table
        // works on the raw bytes and the scan needs no conversion.
        let bytes = path.file_name().unwrap_or_default().as_encoded_bytes();

        let forbidden = Self::forbidden_os(bytes);

        // Linux forbids only '/'. A file name cannot contain '/'.

        if forbidden.is_empty() {
            return ScanResults::one(ScanResult::Ok(path));
        }

        ScanResults::new([
            forbidden
                .contains(OsSet::WINDOWS)
                .then_some(ScanResult::Invalid {
                    path,
                    os: Os::Windows,
                }),
            forbidden
                .contains(OsSet::MACOS)
                .then_some(ScanResult::Invalid {
                    path,
                    os: Os::MacOs,
                }),
        ])
    }

    /// Gives the operating systems that forbid a character in `bytes`.
    ///
    /// This is the fast pass. It runs for each file name in the tree. One
    /// table holds all the operating systems, thus the pass reads each byte
    /// one time.
    fn forbidden_os(bytes: &[u8]) -> OsSet {
        // Examine the bytes and not the characters. Each byte of a character
        // with more than one byte is 128 or more. All the forbidden
        // characters are ASCII. Thus a test of the bytes cannot give a false
        // match, and this pass does not decode the data.
        //
        // The loop has no branch and does no allocation. Almost all file
        // names are correct. Thus the scan usually does no more work.
        let mut result = OsSet::EMPTY;
        for byte in bytes {
            result = result.union(Self::TABLE[*byte as usize]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn assert_invalid(path: &str, expected_characters: &[char], os: Os) {
        let scanner = FilenameScanner::new();
        let path_buf = PathBuf::from(path);
        let results: Vec<_> = scanner.scan(&path_buf).collect();

        let invalid = results.iter().find(
            |result| matches!(result, ScanResult::Invalid { os: result_os, .. } if *result_os == os),
        );

        let Some(result) = invalid else {
            panic!("expected invalid result for {os:?}, got {results:?}");
        };
        let ScanResult::Invalid {
            path: result_path,
            os: result_os,
        } = result
        else {
            unreachable!();
        };

        assert_eq!(*result_path, path_buf);
        assert_eq!(*result_os, os);

        let mut actual: Vec<char> = result.matches().map(|m| m.character).collect();
        actual.sort_unstable();
        let mut expected = expected_characters.to_vec();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn windows() {
        assert_invalid("test?.txt", &['?'], Os::Windows);
        assert_invalid("<test.txt", &['<'], Os::Windows);
        assert_invalid("test>.txt", &['>'], Os::Windows);
        assert_invalid("test:.txt", &[':'], Os::Windows);
        assert_invalid("test|", &['|'], Os::Windows);
        assert_invalid("te\"st.txt", &['"'], Os::Windows);

        // More than one forbidden character in one file name
        assert_invalid("test<>?.txt", &['<', '>', '?'], Os::Windows);
    }

    #[test]
    fn macos() {
        assert_invalid("test:.txt", &[':'], Os::MacOs);

        // More than one colon
        assert_invalid("test:file:name.txt", &[':', ':'], Os::MacOs);
    }

    #[test]
    fn characters_with_more_than_one_byte_give_no_match() {
        // Each byte of these characters is 128 or more. Thus no byte can
        // match a forbidden ASCII character.
        let name = "café_日本語.txt";
        assert!(FilenameScanner::forbidden_os(name.as_bytes()).is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn a_name_that_is_not_valid_utf8_gives_a_match() {
        use std::os::unix::ffi::OsStrExt;

        // The byte 0xFF is not valid UTF-8. The scan reads the raw bytes,
        // thus it finds the colon. The report converts the name later.
        let path = PathBuf::from(std::ffi::OsStr::from_bytes(b"bad\xFF:name.txt"));
        let results: Vec<_> = FilenameScanner::new().scan(&path).collect();

        let invalid = results
            .iter()
            .find(|result| matches!(result, ScanResult::Invalid { os: Os::MacOs, .. }));
        let Some(result) = invalid else {
            panic!("expected invalid result for macOS, got {results:?}");
        };

        // `Matches` converts the name. The lossy conversion replaces the
        // byte 0xFF, and the colon keeps its place.
        let characters: Vec<char> = result.matches().map(|m| m.character).collect();
        assert_eq!(characters, vec![':']);
    }

    #[test]
    fn a_correct_name_gives_one_ok_result() {
        let path = PathBuf::from("/root/ok.txt");
        let results: Vec<_> = FilenameScanner::new().scan(&path).collect();

        assert_eq!(results, vec![ScanResult::Ok(&path)]);
        assert_eq!(results[0].matches().count(), 0);
    }

    #[test]
    fn one_pass_finds_each_operating_system() {
        // The colon is forbidden on Windows and on macOS. One pass over the
        // bytes gives the two results.
        let path = PathBuf::from("test:.txt");
        let results: Vec<_> = FilenameScanner::new().scan(&path).collect();

        assert_eq!(
            results,
            vec![
                ScanResult::Invalid {
                    path: &path,
                    os: Os::Windows
                },
                ScanResult::Invalid {
                    path: &path,
                    os: Os::MacOs
                },
            ]
        );
    }
}
