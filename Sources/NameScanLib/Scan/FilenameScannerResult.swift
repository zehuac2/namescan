//
//  FilenameScannerResult.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation

public enum FilenameScannerResult: Equatable, CustomStringConvertible {
  public enum OS {
    case windows
    case linux
    case macOS
  }

  case ok(url: URL)
  case invalid(url: URL, matches: [Regex<Substring>.Match], os: OS)

  public static func == (lhs: FilenameScannerResult, rhs: FilenameScannerResult) -> Bool {
    switch (lhs, rhs) {
    case (.ok(let lhsURL), .ok(let rhsURL)):
      return lhsURL == rhsURL
    case (
      .invalid(let lhsURL, let lhsMatches, let lhsOS),
      .invalid(let rhsURL, let rhsMatches, let rhsOS)
    ):
      return lhsURL == rhsURL && lhsOS == rhsOS && lhsMatches.count == rhsMatches.count
        && zip(lhsMatches, rhsMatches).allSatisfy { $0.0.output == $0.1.output }
    default:
      return false
    }
  }

  public var description: String {
    switch self {
    case .ok(let url):
      return "OK: \(url)"
    case .invalid(let url, let matches, let os):
      let characters = matches.map { String($0.output) }.joined(separator: ", ")
      return "Invalid: \(url), Characters: \(characters), OS: \(os)"
    }
  }
}
