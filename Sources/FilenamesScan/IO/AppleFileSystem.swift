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
    
    if isDirectory {
      print("is directory")
    }
    
    return []
  }
}
