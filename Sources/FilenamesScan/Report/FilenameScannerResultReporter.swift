//
//  FilenameScannerResultReporter.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation

public protocol FilenameScannerResultReporter {
  mutating func report(_ result: FilenameScannerResult)
  mutating func finish()
}
