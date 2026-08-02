use std::borrow::Cow;
use std::fmt;
use std::io;
use std::path::Path;

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
///
/// The result borrows the path and does not own it. A `PathBuf` copies the
/// full path, thus an owned path made one allocation and one copy for each
/// item in the tree. The reporter reads the path and then the program
/// discards the result, thus the result does not need its own path.
///
/// The result also does not keep the forbidden characters. A `Vec` of the
/// characters made one allocation for each invalid name. [`Self::matches`]
/// finds the characters again.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanResult<'a> {
    Ok(&'a Path),
    Invalid { path: &'a Path, os: Os },
}

impl<'a> ScanResult<'a> {
    /// Gives the forbidden characters in the file name.
    ///
    /// The result does not keep the characters, thus this method reads the
    /// name one more time. Only the report needs the characters, and the
    /// report runs only for an invalid name. A correct name gives no
    /// character.
    pub fn matches(&self) -> Matches<'a> {
        match *self {
            ScanResult::Ok(_) => Matches::new("", OsSet::EMPTY),
            ScanResult::Invalid { path, os } => Matches::new(
                path.file_name().unwrap_or_default().to_string_lossy(),
                OsSet::of(os),
            ),
        }
    }
}

impl fmt::Display for ScanResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanResult::Ok(path) => write!(f, "OK: {}", path.display()),
            ScanResult::Invalid { path, os } => {
                write!(f, "Invalid: {}, Characters: ", path.display())?;
                // Write each character directly. A `Vec` of `String`s and a
                // `join` made more allocations.
                for (position, character_match) in self.matches().enumerate() {
                    if position > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", character_match.character)?;
                }
                write!(f, ", OS: {os}")
            }
        }
    }
}

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
    fn new(text: impl Into<Cow<'a, str>>, os: OsSet) -> Self {
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

/// The results of the scan of one file name.
///
/// The scan gives a maximum of two results: one result for each operating
/// system with a rule. The type holds the results inline, thus
/// [`FilenameScanner::scan`] makes no allocation.
#[derive(Debug, Clone)]
pub struct ScanResults<'a> {
    results: [Option<ScanResult<'a>>; 2],
    /// The index of the next slot in `results`.
    index: usize,
}

impl<'a> ScanResults<'a> {
    /// Makes the results from the slots. An empty slot gives no result.
    fn new(results: [Option<ScanResult<'a>>; 2]) -> Self {
        Self { results, index: 0 }
    }

    /// Makes the results with one result.
    fn one(result: ScanResult<'a>) -> Self {
        Self::new([Some(result), None])
    }
}

impl<'a> Iterator for ScanResults<'a> {
    type Item = ScanResult<'a>;

    fn next(&mut self) -> Option<ScanResult<'a>> {
        while self.index < self.results.len() {
            let result = self.results[self.index];
            self.index += 1;

            if result.is_some() {
                return result;
            }
        }

        None
    }
}

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
    const TABLE: ForbiddenTable = add_to_table(
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
        assert!(FilenameScanner::forbidden_os(name.as_bytes()).is_empty());
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

    /// A record of one reported result. `os` is `None` for a correct name.
    ///
    /// The reporter keeps owned data, because a [`ScanResult`] borrows the
    /// path and does not live longer than the call to `report`.
    type Record = (PathBuf, Option<Os>);

    #[derive(Default)]
    struct RecordingReporter {
        reported: Vec<Record>,
        files: usize,
        finished: bool,
    }

    impl Reporter for RecordingReporter {
        fn report(&mut self, result: &ScanResult<'_>) {
            let record = match result {
                ScanResult::Ok(path) => (path.to_path_buf(), None),
                ScanResult::Invalid { path, os } => (path.to_path_buf(), Some(*os)),
            };
            self.reported.push(record);
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
        let has = |path: &str, os: Option<Os>| {
            reporter
                .reported
                .iter()
                .any(|(result_path, result_os)| result_path == Path::new(path) && *result_os == os)
        };

        assert!(has("/root/sub/bad:.txt", Some(Os::Windows)));
        assert!(has("/root/sub/bad:.txt", Some(Os::MacOs)));
        assert!(has("/root/ok.txt", None));
    }
}
