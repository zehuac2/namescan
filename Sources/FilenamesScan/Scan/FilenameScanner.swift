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

    if let invalidCharacter = findInvalidCharacterOnWindows(name) {
      results.append(.invalid(url: url, character: invalidCharacter, os: .windows))
    }

    if let invalidCharacter = findInvalidCharacterOnMacOS(name) {
      results.append(.invalid(url: url, character: invalidCharacter, os: .macOS))
    }

    if let invalidCharacter = findInvalidCharacterOnLinux(name) {
      results.append(.invalid(url: url, character: invalidCharacter, os: .linux))
    }

    if results.isEmpty {
      results.append(.ok(url: url))
    }

    return results
  }

  private func findInvalidCharacterOnWindows(_ filename: String) -> Character? {
    if let match = filename.firstMatch(of: Self.windowsForbidden) {
      return match.output.first
    }

    return nil
  }

  private func findInvalidCharacterOnMacOS(_ filename: String) -> Character? {
    if let match = filename.firstMatch(of: Self.macOSForbidden) {
      return match.output.first
    }

    return nil
  }

  private func findInvalidCharacterOnLinux(_ filename: String) -> Character? {
    for character in filename {
      switch character {
      // macOS's Finder let you create files with '/' but is is stored as `:`;
      // Both Linux and Windows do not allow '/' in filenames so there is no need to check for it.
      case "/":
        continue
      default:
        continue
      }
    }

    return nil
  }
}
