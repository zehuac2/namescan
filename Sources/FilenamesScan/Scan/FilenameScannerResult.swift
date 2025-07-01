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
  case invalid(url: URL, character: Character, os: OS)

  public var description: String {
    switch self {
    case .ok(let url):
      return "OK: \(url)"
    case .invalid(let url, let character, let os):
      return "Invalid: \(url), Character: \(character), OS: \(os)"
    }
  }
}
