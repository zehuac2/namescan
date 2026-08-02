use super::scan_result::ScanResult;

/// The results of the scan of one file name.
///
/// The scan gives a maximum of two results: one result for each operating
/// system with a rule. The type holds the results inline, thus
/// [`super::filename_scanner::FilenameScanner::scan`] makes no allocation.
#[derive(Debug, Clone)]
pub struct ScanResults<'a> {
    results: [Option<ScanResult<'a>>; 2],
    /// The index of the next slot in `results`.
    index: usize,
}

impl<'a> ScanResults<'a> {
    /// Makes the results from the slots. An empty slot gives no result.
    pub(super) fn new(results: [Option<ScanResult<'a>>; 2]) -> Self {
        Self { results, index: 0 }
    }

    /// Makes the results with one result.
    pub(super) fn one(result: ScanResult<'a>) -> Self {
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
