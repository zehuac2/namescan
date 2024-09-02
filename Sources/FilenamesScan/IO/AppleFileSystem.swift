//
//  AppleFileSystem.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation

public struct AppleFileSystem: FileSystem {
  public let fileManager: FileManager = FileManager()

  public var currentDirectory: URL {
    return URL(fileURLWithPath: fileManager.currentDirectoryPath)
  }

  public init() {}

  public func listDir(_ url: URL) throws -> [URL] {
    let resourceValues = try url.resourceValues(forKeys: [.isDirectoryKey])
    let isDirectory = resourceValues.isDirectory ?? false

    guard isDirectory else {
      return []
    }

    var urls = [URL]()

    if let enumerator = fileManager.enumerator(at: url, includingPropertiesForKeys: nil) {
      for item in enumerator {
        if let url = item as? URL {
          urls.append(url)
        }
      }
    }

    return urls
  }
}
