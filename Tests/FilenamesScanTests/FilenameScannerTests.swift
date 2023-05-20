//
//  File.swift
//  
//
//  Created by Zehua Chen on 5/20/23.
//

import XCTest

@testable import FilenamesScan

final class FilenameScannerTests: XCTestCase {
  func testExample() throws {
    let filenameScanner = FilenameScanner()
    let url = URL(string: "file:///name|")!
    let results = filenameScanner.scan(url)
    
    XCTAssertEqual(results, [.invalid(url: url, os: .windows)])
  }
}
