//
//  FilenameScannerResult.swift
//  
//
//  Created by Zehua Chen on 5/20/23.
//

import Foundation

public enum FilenameScannerResult {
  public enum OS {
    case windows
  }
  
  case invalid(os: OS)
}
