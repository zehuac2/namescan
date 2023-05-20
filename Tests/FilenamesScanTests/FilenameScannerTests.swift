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
    let invalidURLS = [
      URL(string: "file://Root/Foo%2022%5C22.pdf")!
    ]
    
    let filenameScanner = FilenameScanner()
    
    for invalidURL in invalidURLS {
      let results = filenameScanner.scan(invalidURL)
      XCTAssertEqual(results, [.invalid(url: invalidURL, os: .windows)])
    }
  }
}
