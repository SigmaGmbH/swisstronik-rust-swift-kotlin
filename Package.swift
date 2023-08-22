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
        .binaryTarget(name: "SwisstronikRust", path: "./swisstronik/SwisstronikRust.xcframework"),
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
