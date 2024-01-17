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
                      checksum: "fc8e8f002de250a2cb6c11c3c246462c9448be777e76df73b6750085888dd084"
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
