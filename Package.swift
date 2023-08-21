// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: swisstronik-swift

import PackageDescription;

let package = Package(
    name: "swisstronik-swift",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_10),
    ],
    products: [
        .library(
            name: "swisstronik-swift",
            targets: ["swisstronik-swift"]
        )
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-protobuf.git", from: "1.6.0"),
    ],
    targets: [
        .binaryTarget(name: "SwisstronikRust", path: "./swisstronik/SwisstronikRust.xcframework"),
        .target(
            name: "swisstronik-swift",
            dependencies: [
                .target(name: "SwisstronikRust"),
                .product(name: "SwiftProtobuf", package: "swift-protobuf")
            ],
            path: ".",
            sources: [
                "swift/SwisstronikSwift.swift",
                "swift/protobuf_contracts",
            ],
            publicHeadersPath: "include",
            cSettings: [
                .headerSearchPath("include/"),
            ]
        )
    ]
)
