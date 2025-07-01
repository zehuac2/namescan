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
    switch result {
    case .invalid(let url, let matches, let os):
      if isLastLineSuccess {
        print("\r", terminator: "")
        isLastLineSuccess = false
      }
      let characters = matches.map { String($0.output) }.joined(separator: ", ")
      print("Invalid \(os): \(url.relativePath); \(characters) forbidden.")
    default:
      break
    }
  }

  public mutating func finishFile() {
    count += 1

    if count % reportIncrement == 0 {
      print("\r\(count) items scanned", terminator: "")
      isLastLineSuccess = true
    }
  }

  public func finish() {
    print()
    print("Finished. \(count) items scanned")
  }
}
