//
//  FilenameScanner.swift
//  
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation

struct FilenameScanner {
  init() {}
  
  func scan(_ url: URL) -> [FilenameScannerResult] {
    let name = url.lastPathComponent
    var results = [FilenameScannerResult]()
    
    if isValidOnWindows(name) {
      results.append(.invalid(url: url, os: .windows))
    }
    
    if isValidOnLinux(name) {
      results.append(.invalid(url: url, os: .linux))
    }
    
    if results.isEmpty {
      results.append(.ok(url: url))
    }
    
    return results
  }
  
  private func isValidOnWindows(_ filename: String) -> Bool {
    return filename.contains { char in
      switch char {
      case "<", ">", ":", "\"", "/", "\\", "|", "?", "*":
        return true
      default:
        return false
      }
    }
  }
  
  private func isValidOnLinux(_ filename: String) -> Bool {
    return filename.contains { char in
      switch char {
      case "/":
        return true
      default:
        return false
      }
    }
  }
}
