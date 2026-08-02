use crate::scan::ScanResult;

/// A reporter that receives the [`ScanResult`]s of a directory scan.
pub trait Reporter {
    /// Reports the scan result of one file name.
    fn report(&mut self, result: &ScanResult<'_>);
    /// Ends the scan of one file or directory.
    fn finish_file(&mut self);
    /// Ends the full scan.
    fn finish(&mut self);
}
