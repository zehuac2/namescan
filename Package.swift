// swift-tools-version: 5.8
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "namescan",
  platforms: [.macOS("15.0")],
  products: [
    // Products define the executables and libraries a package produces, making them visible to other packages.
    .executable(
      name: "namescan",
      targets: ["namescan"])
  ],
  dependencies: [
    .package(url: "https://github.com/apple/swift-argument-parser", from: "1.5.1")
  ],
  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .target(name: "NameScanLib"),
    .executableTarget(
      name: "namescan",
      dependencies: [
        "NameScanLib",
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
      ]),
    .testTarget(
      name: "namescanTests",
      dependencies: ["namescan"]),
    .testTarget(
      name: "NameScanLibTests",
      dependencies: ["NameScanLib"]),
  ]
)
