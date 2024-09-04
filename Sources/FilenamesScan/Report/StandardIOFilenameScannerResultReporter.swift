//
//  StandardIOFilenameScannerResultReporter.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation

public struct StandardIOFilenameScannerResultReporter: FilenameScannerResultReporter {
  let reportIncrement: Int
  private var count: Int = 0
  private var isLastLineSuccess: Bool = false

  public init(reportIncrement: Int) {
    self.reportIncrement = reportIncrement
  }

  public mutating func report(_ result: FilenameScannerResult) {
    count += 1

    if count % reportIncrement == 0 {
      print("\r\(count) items scanned", terminator: "")
      isLastLineSuccess = true
    }

    switch result {
    case .invalid(let url, let os):
      if isLastLineSuccess {
        print("\r", terminator: "")
        isLastLineSuccess = false
      }
      print("Invalid \(os): \(url.relativePath)")
    default:
      break
    }
  }

  public func finish() {
    print()
    print("Finished. \(count) items scanned")
  }
}
