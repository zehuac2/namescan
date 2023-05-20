//
//  FileSystem.swift
//  
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation

public protocol FileSystem {
  func listDir(_ url: URL) throws -> [URL]
}

public extension FileSystem where Self == AppleFileSystem {
  static var apple: AppleFileSystem {
    return .init()
  }
}
