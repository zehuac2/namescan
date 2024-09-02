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

extension FileSystem where Self == AppleFileSystem {
  public static var apple: AppleFileSystem {
    return .init()
  }
}
