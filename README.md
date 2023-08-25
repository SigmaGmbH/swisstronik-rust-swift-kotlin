# Rust core for Kotlin and Swift Swisstronik libraries

This is the repo for shared Rust code to be used in Swisstronik Swift & Kotlin.

## Usage:


### Swift using `swift build`

Add it to your dependencies in Package.swift:

```swift

dependencies: [
    .package(url: "https://github.com/SigmaGmbH/swisstronik-rust-swift-kotlin", from: "1.0.0"),
],
```

Then add the library to target's `dependencies`:

```swift
.product(name: "SwisstronikSwift", package: "SwisstronikSwift"),
```

or, add it using XCode:


### Swift using Xcode

If you are using Xcode, then you should:
* Add this SwiftPM package `https://github.com/SigmaGmbH/swisstronik-rust-swift-kotlin`  as dependency of your xcode project:
  [Apple Docs](https://developer.apple.com/documentation/swift_packages/adding_package_dependencies_to_your_app)

### Kotlin

The package is hosted on Github Packages. In order to use it, you need to authenticate on Github using your token.

Here's a guide to working with Github Packages:

https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-gradle-registry

Here's the short version:

1. Add to your Gradle `dependencies`:

```groovy
dependencies {
    implementation "com.swisstronik.kotlin:1.0.0"
}
```

2. Add the repository to your main `build.gradle` in `repositories` section:

```groovy
repositories {
    maven {
        url = uri("https://maven.pkg.github.com/SigmaGmbH/swisstronik-rust-swift-kotlin")
        credentials {
            username = project.findProperty("gpr.user") ?: System.getenv("USERNAME")
            password = project.findProperty("gpr.key") ?: System.getenv("TOKEN")
        }
   }
```

Also, you can check out [TrustWallet guide](https://developer.trustwallet.com/developer/wallet-core/integration-guide/android-guide#adding-library-dependency) for detailed explanation.
Don't forget to replace the link with `https://maven.pkg.github.com/SigmaGmbH/swisstronik-rust-swift-kotlin` !

## Dependencies

* https://github.com/mozilla/cbindgen   -  `cbindgen binary`
* https://github.com/apple/swift-protobuf - `swift code generation plugin`
* https://protobuf.dev/ - `protoc binary`
  
### Commands

Command runner - https://github.com/casey/just

Generate C header file for Swift:

`just bindings`

Generate Kotlin protobuf files:

`just kotlinpb`

Generate Swift protobuf files:

`just swiftpb`
