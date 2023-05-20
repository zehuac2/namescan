//
//  main.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation
import FilenamesScan

let reporter = StandardIOFilenameScannerResultReporter()
let fileSystem = AppleFileSystem()
let directoryScanner = DirectoryScanner(fileSystem: fileSystem, reporter: reporter)

try! directoryScanner.scan(fileSystem.currentDirectory)
