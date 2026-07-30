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

/// The scanner that finds forbidden characters in file names.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilenameScanner;

impl FilenameScanner {
    /// Characters forbidden in Windows file names.
    pub const WINDOWS_FORBIDDEN: &'static [char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    /// Characters forbidden in macOS file names.
    pub const MACOS_FORBIDDEN: &'static [char] = &[':'];

    pub fn new() -> Self {
        Self
    }

    /// Scans the file name of `path`.
    ///
    /// The function returns one [`ScanResult::Invalid`] for each violated
    /// file name rule. The function returns one [`ScanResult::Ok`] when the
    /// file name has no forbidden characters.
    pub fn scan(&self, path: &Path) -> Vec<ScanResult> {
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();

        let mut results = Vec::new();

        let windows_matches = Self::find_forbidden(&name, Self::WINDOWS_FORBIDDEN);
        if !windows_matches.is_empty() {
            results.push(ScanResult::Invalid {
                path: path.to_path_buf(),
                matches: windows_matches,
                os: Os::Windows,
            });
        }

        let macos_matches = Self::find_forbidden(&name, Self::MACOS_FORBIDDEN);
        if !macos_matches.is_empty() {
            results.push(ScanResult::Invalid {
                path: path.to_path_buf(),
                matches: macos_matches,
                os: Os::MacOs,
            });
        }

        // Linux forbids only '/'. A file name cannot contain '/'.

        if results.is_empty() {
            results.push(ScanResult::Ok(path.to_path_buf()));
        }

        results
    }

    fn find_forbidden(name: &str, forbidden: &[char]) -> Vec<CharMatch> {
        name.chars()
            .enumerate()
            .filter(|(_, c)| forbidden.contains(c))
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
        let mut to_visit = vec![root.to_path_buf()];
        let filename_scanner = FilenameScanner::new();

        while let Some(path) = to_visit.pop() {
            let children = self.file_system.list_dir(&path)?;
            to_visit.extend(children);

            for result in filename_scanner.scan(&path) {
                self.reporter.report(&result);
            }

            self.reporter.finish_file();
        }

        self.reporter.finish();
        Ok(())
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
            FilenameScanner::find_forbidden("\\test.txt", FilenameScanner::WINDOWS_FORBIDDEN);
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
    fn macos() {
        assert_invalid("test:.txt", &[':'], Os::MacOs);

        // More than one colon
        assert_invalid("test:file:name.txt", &[':', ':'], Os::MacOs);
    }

    struct MockFileSystem {
        children: HashMap<PathBuf, Vec<PathBuf>>,
    }

    impl FileSystem for MockFileSystem {
        fn list_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
            Ok(self.children.get(path).cloned().unwrap_or_default())
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
            vec![PathBuf::from("/root/ok.txt"), PathBuf::from("/root/sub")],
        );
        children.insert(
            PathBuf::from("/root/sub"),
            vec![PathBuf::from("/root/sub/bad:.txt")],
        );

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
