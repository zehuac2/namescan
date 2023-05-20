//
//  FilenameScannerResultReporter.swift
//  
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation

public protocol FilenameScannerResultReporter {
  func report(_ result: FilenameScannerResult)
}
