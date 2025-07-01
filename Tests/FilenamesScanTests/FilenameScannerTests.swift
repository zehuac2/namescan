//
//  File.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//
import Foundation
import Testing

@testable import FilenamesScan

@Suite("FilenameScannerTests")
struct FilenameScannerTests {
  private func testPath(_ path: String, expectedCharacters: [String], os: FilenameScannerResult.OS)
  {
    let url = URL(filePath: path)
    let filenameScanner = FilenameScanner()
    let results = filenameScanner.scan(url)

    // Find the result for the specific OS
    let invalidResult = results.first { result in
      if case .invalid(_, _, let resultOS) = result, resultOS == os {
        return true
      }
      return false
    }

    guard case .invalid(let resultURL, let matches, let resultOS) = invalidResult else {
      #expect(Bool(false), "Expected invalid result for \(os)")
      return
    }

    #expect(resultURL == url)
    #expect(resultOS == os)

    let actualCharacters = matches.map { String($0.output) }
    #expect(Set(actualCharacters) == Set(expectedCharacters))
  }

  @Test func windows() async throws {
    testPath("\\test.txt", expectedCharacters: ["\\"], os: .windows)
    testPath("test?.txt", expectedCharacters: ["?"], os: .windows)
    testPath("<test.txt", expectedCharacters: ["<"], os: .windows)
    testPath("test>.txt", expectedCharacters: [">"], os: .windows)
    testPath("test:.txt", expectedCharacters: [":"], os: .windows)
    testPath("test|", expectedCharacters: ["|"], os: .windows)
    testPath("te\"st.txt", expectedCharacters: ["\""], os: .windows)

    // Test multiple invalid characters in one filename
    testPath("test<>?.txt", expectedCharacters: ["<", ">", "?"], os: .windows)
  }

  @Test func macOS() async throws {
    testPath("test:.txt", expectedCharacters: [":"], os: .macOS)

    // Test multiple colons
    testPath("test:file:name.txt", expectedCharacters: [":", ":"], os: .macOS)
  }
}
