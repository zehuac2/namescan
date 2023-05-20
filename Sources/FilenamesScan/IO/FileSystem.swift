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
