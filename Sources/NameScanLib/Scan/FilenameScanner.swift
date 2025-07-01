//
//  FilenameScanner.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation
import RegexBuilder

struct FilenameScanner {
  static let windowsForbidden: Regex<Substring> = #/[<>:"/\\|?*]/#
  static let macOSForbidden: Regex<Substring> = #/[:]/#
  init() {}

  func scan(_ url: URL) -> [FilenameScannerResult] {
    let name = url.lastPathComponent
    var results = [FilenameScannerResult]()

    let windowsMatches = findInvalidCharactersOnWindows(name)
    if !windowsMatches.isEmpty {
      results.append(.invalid(url: url, matches: windowsMatches, os: .windows))
    }

    let macOSMatches = findInvalidCharactersOnMacOS(name)
    if !macOSMatches.isEmpty {
      results.append(.invalid(url: url, matches: macOSMatches, os: .macOS))
    }

    let linuxMatches = findInvalidCharactersOnLinux(name)
    if !linuxMatches.isEmpty {
      results.append(.invalid(url: url, matches: linuxMatches, os: .linux))
    }

    if results.isEmpty {
      results.append(.ok(url: url))
    }

    return results
  }

  private func findInvalidCharactersOnWindows(_ filename: String) -> [Regex<Substring>.Match] {
    return filename.matches(of: Self.windowsForbidden)
  }

  private func findInvalidCharactersOnMacOS(_ filename: String) -> [Regex<Substring>.Match] {
    return filename.matches(of: Self.macOSForbidden)
  }

  private func findInvalidCharactersOnLinux(_ filename: String) -> [Regex<Substring>.Match] {
    // For Linux, we currently don't have any forbidden characters in the regex
    // The logic was just checking for '/' which is allowed on Linux
    // So we return an empty array for now
    return []
  }
}
