package com.swisstronik.kotlin

open class SwisstronikKotlin {
    init {
        try {
            System.loadLibrary("swisstronik")
        } catch (e: UnsatisfiedLinkError) {
            throw UnsatisfiedLinkError("Error linking Swisstronik Rust library. Check that the .so file " +
                "for the current architecture is in the libs directory. Error: $e")
        }
    }

    external fun initLogger()
    external fun call(input: ByteArray): ByteArray
}