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
  private func testPath(_ path: String, invalidCharacter: Character, os: FilenameScannerResult.OS) {
    let url = URL(filePath: path)
    let filenameScanner = FilenameScanner()
    let results = filenameScanner.scan(url)

    #expect(results == [.invalid(url: url, character: invalidCharacter, os: os)])
  }

  @Test func windows() async throws {
    testPath("\\test.txt", invalidCharacter: "\\", os: .windows)
    testPath("test?.txt", invalidCharacter: "?", os: .windows)
    testPath("<test.txt", invalidCharacter: "<", os: .windows)
    testPath("test>.txt", invalidCharacter: ">", os: .windows)
    testPath("test:.txt", invalidCharacter: ":", os: .windows)
    testPath("test|", invalidCharacter: "|", os: .windows)
    testPath("te\"st.txt", invalidCharacter: "\"", os: .windows)
  }

  @Test func linux() async throws {
    testPath("test/test.txt", invalidCharacter: "/", os: .linux)
  }
}
