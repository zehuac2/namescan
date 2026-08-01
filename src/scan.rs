use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use crate::io::FileSystem;
use crate::report::Reporter;

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

/// The result of the scan of a single file name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanResult {
    Ok(PathBuf),
    Invalid {
        path: PathBuf,
        matches: Vec<CharMatch>,
        os: Os,
    },
}

impl fmt::Display for ScanResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanResult::Ok(path) => write!(f, "OK: {}", path.display()),
            ScanResult::Invalid { path, matches, os } => {
                let characters = matches
                    .iter()
                    .map(|m| m.character.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(
                    f,
                    "Invalid: {}, Characters: {}, OS: {}",
                    path.display(),
                    characters,
                    os
                )
            }
        }
    }
}

/// A table that tells for each byte if the byte is a forbidden character.
type ForbiddenTable = [bool; 256];

/// Makes a [`ForbiddenTable`] from a list of forbidden characters.
///
/// The table gives a test that takes a constant time. A search with
/// `slice::contains` compares the character with each character in the list.
///
/// The function runs at compile time. It stops the build if a character is
/// not ASCII, because the table holds one entry for each byte value.
const fn make_table(forbidden: &[char]) -> ForbiddenTable {
    let mut table = [false; 256];
    let mut index = 0;

    while index < forbidden.len() {
        let character = forbidden[index];
        assert!(character.is_ascii(), "a forbidden character must be ASCII");
        table[character as usize] = true;
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

    /// The table for [`Self::WINDOWS_FORBIDDEN`].
    const WINDOWS_TABLE: ForbiddenTable = make_table(Self::WINDOWS_FORBIDDEN);
    /// The table for [`Self::MACOS_FORBIDDEN`].
    const MACOS_TABLE: ForbiddenTable = make_table(Self::MACOS_FORBIDDEN);

    pub fn new() -> Self {
        Self
    }

    /// Scans the file name of `path`.
    ///
    /// The function returns one [`ScanResult::Invalid`] for each violated
    /// file name rule. The function returns one [`ScanResult::Ok`] when the
    /// file name has no forbidden characters.
    pub fn scan(&self, path: &Path) -> Vec<ScanResult> {
        let name = path.file_name().unwrap_or_default();

        // Examine the raw bytes of the name. `OsStr::to_string_lossy` makes
        // a `String` for each file name, also for a correct name. The bytes
        // of an `OsStr` hold each ASCII character as itself on all the
        // platforms. All the forbidden characters are ASCII. Thus the tables
        // work on the raw bytes and the scan needs no conversion.
        let bytes = name.as_encoded_bytes();

        let has_windows_match = Self::has_forbidden(bytes, &Self::WINDOWS_TABLE);
        let has_macos_match = Self::has_forbidden(bytes, &Self::MACOS_TABLE);

        // Linux forbids only '/'. A file name cannot contain '/'.

        if !has_windows_match && !has_macos_match {
            return vec![ScanResult::Ok(path.to_path_buf())];
        }

        // Make the text of the name here. Only a name that has a forbidden
        // character comes to this point, and the report needs the
        // characters.
        let text = name.to_string_lossy();
        let mut results = Vec::new();

        if has_windows_match {
            results.push(ScanResult::Invalid {
                path: path.to_path_buf(),
                matches: Self::collect_forbidden(&text, &Self::WINDOWS_TABLE),
                os: Os::Windows,
            });
        }

        if has_macos_match {
            results.push(ScanResult::Invalid {
                path: path.to_path_buf(),
                matches: Self::collect_forbidden(&text, &Self::MACOS_TABLE),
                os: Os::MacOs,
            });
        }

        results
    }

    /// Tells if `bytes` holds a character that `table` forbids.
    ///
    /// This is the fast pass. It runs for each file name in the tree.
    fn has_forbidden(bytes: &[u8], table: &ForbiddenTable) -> bool {
        // Examine the bytes and not the characters. Each byte of a character
        // with more than one byte is 128 or more. All the forbidden
        // characters are ASCII. Thus a test of the bytes cannot give a false
        // match, and this pass does not decode the data.
        //
        // The loop has no branch and does no allocation. Almost all file
        // names are correct. Thus the scan usually does no more work.
        let mut result = false;
        for byte in bytes {
            result |= table[*byte as usize];
        }

        result
    }

    /// Collects the characters in `name` that `table` forbids.
    ///
    /// This is the slow pass. It runs only for a file name that has a
    /// forbidden character. Thus its speed is not important.
    fn collect_forbidden(name: &str, table: &ForbiddenTable) -> Vec<CharMatch> {
        // This pass decodes the characters, because `CharMatch::index`
        // counts the characters and does not count the bytes.
        name.chars()
            .enumerate()
            .filter(|(_, c)| c.is_ascii() && table[*c as usize])
            .map(|(index, character)| CharMatch { character, index })
            .collect()
    }
}

/// The scanner that scans a full directory tree.
///
/// The scanner sends the [`ScanResult`] of each visited file and directory
/// to the reporter.
pub struct DirectoryScanner<FS: FileSystem, R: Reporter> {
    pub file_system: FS,
    pub reporter: R,
}

impl<FS: FileSystem, R: Reporter> DirectoryScanner<FS, R> {
    pub fn new(file_system: FS, reporter: R) -> Self {
        Self {
            file_system,
            reporter,
        }
    }

    pub fn scan(&mut self, root: &Path) -> io::Result<()> {
        let filename_scanner = FilenameScanner::new();

        // Scan the name of the root. The loop below scans only the children.
        self.scan_name(&filename_scanner, root);

        let mut to_visit = vec![root.to_path_buf()];

        while let Some(directory) = to_visit.pop() {
            for entry in self.file_system.list_dir(&directory)? {
                self.scan_name(&filename_scanner, &entry.path);

                // Visit only the children that are directories. The file
                // system gives the type of each child. Thus the scanner does
                // no system call to find the type.
                if entry.is_dir {
                    to_visit.push(entry.path);
                }
            }
        }

        self.reporter.finish();
        Ok(())
    }

    /// Scans the file name of `path` and sends the results to the reporter.
    fn scan_name(&mut self, filename_scanner: &FilenameScanner, path: &Path) {
        for result in filename_scanner.scan(path) {
            self.reporter.report(&result);
        }

        self.reporter.finish_file();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn assert_invalid(path: &str, expected_characters: &[char], os: Os) {
        let scanner = FilenameScanner::new();
        let path_buf = PathBuf::from(path);
        let results = scanner.scan(&path_buf);

        let invalid = results.iter().find(
            |result| matches!(result, ScanResult::Invalid { os: result_os, .. } if *result_os == os),
        );

        let Some(ScanResult::Invalid {
            path: result_path,
            matches,
            os: result_os,
        }) = invalid
        else {
            panic!("expected invalid result for {os:?}, got {results:?}");
        };

        assert_eq!(*result_path, path_buf);
        assert_eq!(*result_os, os);

        let mut actual: Vec<char> = matches.iter().map(|m| m.character).collect();
        actual.sort_unstable();
        let mut expected = expected_characters.to_vec();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn windows() {
        // `Path::file_name` treats '\\' as a separator on Windows, so a
        // `PathBuf` cannot carry a file name that contains it. Test the
        // character-detection logic directly on the raw name instead.
        let matches =
            FilenameScanner::collect_forbidden("\\test.txt", &FilenameScanner::WINDOWS_TABLE);
        assert_eq!(
            matches,
            vec![CharMatch {
                character: '\\',
                index: 0
            }]
        );

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
    fn characters_with_more_than_one_byte_give_no_match() {
        // Each byte of these characters is 128 or more. Thus no byte can
        // match a forbidden ASCII character.
        let name = "café_日本語.txt";
        assert!(!FilenameScanner::has_forbidden(
            name.as_bytes(),
            &FilenameScanner::WINDOWS_TABLE
        ));
    }

    #[test]
    fn index_counts_the_characters_and_not_the_bytes() {
        // The character 'é' has two bytes. The index of ':' is 1 and not 2.
        let matches = FilenameScanner::collect_forbidden("é:x", &FilenameScanner::WINDOWS_TABLE);
        assert_eq!(
            matches,
            vec![CharMatch {
                character: ':',
                index: 1
            }]
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_name_that_is_not_valid_utf8_gives_a_match() {
        use std::os::unix::ffi::OsStrExt;

        // The byte 0xFF is not valid UTF-8. The scan reads the raw bytes,
        // thus it finds the colon. The report converts the name later.
        let path = PathBuf::from(std::ffi::OsStr::from_bytes(b"bad\xFF:name.txt"));
        let results = FilenameScanner::new().scan(&path);

        assert!(
            results
                .iter()
                .any(|result| matches!(result, ScanResult::Invalid { os: Os::MacOs, .. })),
            "got {results:?}"
        );
    }

    #[test]
    fn macos() {
        assert_invalid("test:.txt", &[':'], Os::MacOs);

        // More than one colon
        assert_invalid("test:file:name.txt", &[':', ':'], Os::MacOs);
    }

    use crate::io::DirEntry;

    struct MockFileSystem {
        children: HashMap<PathBuf, Vec<DirEntry>>,
    }

    impl FileSystem for MockFileSystem {
        fn list_dir(&self, path: &Path) -> io::Result<Vec<DirEntry>> {
            Ok(self.children.get(path).cloned().unwrap_or_default())
        }
    }

    fn file(path: &str) -> DirEntry {
        DirEntry {
            path: PathBuf::from(path),
            is_dir: false,
        }
    }

    fn directory(path: &str) -> DirEntry {
        DirEntry {
            path: PathBuf::from(path),
            is_dir: true,
        }
    }

    #[derive(Default)]
    struct RecordingReporter {
        reported: Vec<ScanResult>,
        files: usize,
        finished: bool,
    }

    impl Reporter for RecordingReporter {
        fn report(&mut self, result: &ScanResult) {
            self.reported.push(result.clone());
        }

        fn finish_file(&mut self) {
            self.files += 1;
        }

        fn finish(&mut self) {
            self.finished = true;
        }
    }

    #[test]
    fn directory_scanner_visits_every_item() {
        let root = PathBuf::from("/root");
        let mut children = HashMap::new();
        children.insert(
            root.clone(),
            vec![file("/root/ok.txt"), directory("/root/sub")],
        );
        children.insert(PathBuf::from("/root/sub"), vec![file("/root/sub/bad:.txt")]);

        let mut scanner =
            DirectoryScanner::new(MockFileSystem { children }, RecordingReporter::default());
        scanner.scan(&root).unwrap();

        let reporter = &scanner.reporter;
        assert_eq!(reporter.files, 4);
        assert!(reporter.finished);
        assert!(
            reporter
                .reported
                .iter()
                .any(|r| matches!(
                    r,
                    ScanResult::Invalid { path, os: Os::Windows, .. }
                        if path == Path::new("/root/sub/bad:.txt")
                ))
        );
        assert!(
            reporter
                .reported
                .iter()
                .any(|r| matches!(
                    r,
                    ScanResult::Invalid { path, os: Os::MacOs, .. }
                        if path == Path::new("/root/sub/bad:.txt")
                ))
        );
        assert!(
            reporter
                .reported
                .iter()
                .any(|r| matches!(r, ScanResult::Ok(path) if path == Path::new("/root/ok.txt")))
        );
    }
}
