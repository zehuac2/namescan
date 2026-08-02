use std::io;
use std::path::Path;

use crate::io::FileSystem;
use crate::report::Reporter;

use super::filename_scanner::FilenameScanner;

/// The scanner that scans a full directory tree.
///
/// The scanner sends the [`super::scan_result::ScanResult`] of each visited
/// file and directory to the reporter.
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
    use crate::io::DirEntry;
    use crate::report::Reporter;
    use crate::scan::{Os, ScanResult};
    use std::collections::HashMap;
    use std::path::PathBuf;

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
