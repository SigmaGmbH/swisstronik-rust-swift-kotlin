// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: SwisstronikSwift

import PackageDescription;

let package = Package(
    name: "SwisstronikSwift",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_10),
    ],
    products: [
        .library(
            name: "SwisstronikSwift",
            targets: ["SwisstronikSwift"]
        )
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-protobuf.git", from: "1.6.0"),
    ],
    targets: [
        .binaryTarget(name: "SwisstronikRust",
                      url: "https://github.com/SigmaGmbH/swisstronik-rust-swift-kotlin/releases/download/v1.0.1/SwisstronikRust.xcframework.zip",
                      checksum: "63c5f8767e0504730d7829e05074bf703a5e54be1be1a111877f9961c4887513"
                     ),
        .target(
            name: "SwisstronikSwift",
            dependencies: [
                .target(name: "SwisstronikRust"),
                .product(name: "SwiftProtobuf", package: "swift-protobuf")
            ],
            path: ".",
            sources: [
                "swift/SwisstronikSwift.swift",
                "swift/protobuf_contracts",
            ],
            publicHeadersPath: "include/swisstronik.h"
        ),
        .testTarget(
            name: "SwisstronikSwift-tests",
            dependencies: [
                .target(name: "SwisstronikRust"),
                "SwisstronikSwift"
            ],
            path: "swift/tests"
        )
    ]
)
