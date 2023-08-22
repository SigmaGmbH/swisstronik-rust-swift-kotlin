# Rust core for Kotlin and Swift Swisstronik libraries

This is the repo for shared Rust code to be used in Swisstronik Swift & Kotlin.


### Commands

`cbindgen --config cbindgen.toml --crate swisstronik-rust-swift-kotlin --output include/swisstronik.h`

`protoc --swift_out=./ProtobufGenerated/ protobuf_contracts/contract.proto`

`protoc --java_out=kotlin/swisstronik-kotlin/src/main/java --kotlin_out=kotlin/swisstronik-kotlin/src/main/java protobuf_contracts/contract.proto`