//
//  File.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//

import XCTest

@testable import FilenamesScan

final class FilenameScannerTests: XCTestCase {
  func testWindows() throws {
    testPath("\\test.txt", invalidCharacter: "\\", os: .windows)
    testPath("test?.txt", invalidCharacter: "?", os: .windows)
    testPath("<test.txt", invalidCharacter: "<", os: .windows)
    testPath("test>.txt", invalidCharacter: ">", os: .windows)
    testPath("test:.txt", invalidCharacter: ":", os: .windows)
    testPath("test|", invalidCharacter: "|", os: .windows)
    testPath("te\"st.txt", invalidCharacter: "\"", os: .windows)
  }
  
  func testPath(_ path: String, invalidCharacter: Character, os: FilenameScannerResult.OS) {
    let url = URL(filePath: path)
    let filenameScanner = FilenameScanner()
    let results = filenameScanner.scan(url)
    
    XCTAssertEqual(results, [.invalid(url: url, character: invalidCharacter, os: .windows)])
  }
}
