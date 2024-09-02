// swift-tools-version: 5.8
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "FilenamesScan",
  products: [
    // Products define the executables and libraries a package produces, making them visible to other packages.
    .executable(
      name: "filenames-scan",
      targets: ["filenames-scan"])
  ],
  dependencies: [
    .package(url: "https://github.com/apple/swift-argument-parser", from: "1.5.0")
  ],
  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .target(name: "FilenamesScan"),
    .executableTarget(
      name: "filenames-scan",
      dependencies: [
        "FilenamesScan",
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
      ]),
    .testTarget(
      name: "filenames-scanTests",
      dependencies: ["filenames-scan"]),
    .testTarget(
      name: "FilenamesScanTests",
      dependencies: ["FilenamesScan"]),
  ]
)
