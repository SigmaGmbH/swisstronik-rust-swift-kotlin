# just manual: https://github.com/casey/just#readme
_default:
  just --list

bindings:
  #!/bin/bash -eux
  cbindgen --config cbindgen.toml --crate swisstronik-rust-swift-kotlin --output include/swisstronik.h

swiftpb:
  protoc --swift_out=swift/ protobuf_contracts/contract.proto

kotlinpb:
  protoc --java_out=kotlin/swisstronik-kotlin/src/main/java --kotlin_out=kotlin/swisstronik-kotlin/src/main/java protobuf_contracts/contract.proto

