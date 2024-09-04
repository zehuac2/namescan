//
//  DirectoryScanner.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation

public struct DirectoryScanner<FS: FileSystem, Reporter: FilenameScannerResultReporter> {
  public let fileSystem: FS
  public var reporter: Reporter

  public init(fileSystem: FS, reporter: Reporter) {
    self.fileSystem = fileSystem
    self.reporter = reporter
  }

  public mutating func scan(_ root: URL) throws {
    var toVisits = [root]
    let scanner = FilenameScanner()

    while toVisits.count > 0 {
      let toVisit = toVisits.remove(at: toVisits.count - 1)
      let children = try fileSystem.listDir(toVisit)

      toVisits.append(contentsOf: children)

      let results = scanner.scan(toVisit)

      for result in results {
        reporter.report(result)
      }
    }

    reporter.finish()
  }
}
