//
//  main.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//

import ArgumentParser
import FilenamesScan
import Foundation

@main
struct FilenamesScan: ParsableCommand {
  static let configuration: CommandConfiguration = .init(
    abstract: "Filenames scanner to detect file names that cannot be synced between OS")

  func run() throws {
    let reporter = StandardIOFilenameScannerResultReporter()
    let fileSystem = AppleFileSystem()
    let directoryScanner = DirectoryScanner(fileSystem: fileSystem, reporter: reporter)

    try! directoryScanner.scan(fileSystem.currentDirectory)

    print("Scans finished!")

  }
}
