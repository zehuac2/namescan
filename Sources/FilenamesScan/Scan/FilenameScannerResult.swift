//
//  FilenameScannerResult.swift
//
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation

public enum FilenameScannerResult: Equatable {
  public enum OS {
    case windows
    case linux
  }

  case ok(url: URL)
  case invalid(url: URL, character: Character, os: OS)
}
