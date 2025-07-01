//
//  main.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//

import ArgumentParser
import Foundation
import NameScanLib

@main
struct NameScan: ParsableCommand {
  static let configuration: CommandConfiguration = .init(
    commandName: "namescan",
    abstract: "File name scanner to detect file names that cannot be synced between OS")

  @Argument
  var path: String = "."

  @Option(name: .shortAndLong)
  var reportIncrement: Int = 100

  func run() throws {
    let reporter = StandardIOFilenameScannerResultReporter(reportIncrement: 100)
    let fileSystem = AppleFileSystem()
    var directoryScanner = DirectoryScanner(fileSystem: fileSystem, reporter: reporter)

    try directoryScanner.scan(URL(filePath: path))
  }
}
