package com.swisstronik.kotlin

object AndroidTestUtils {

    @JvmOverloads
    fun ByteArray.toHexString(separator: CharSequence = " ",  prefix: CharSequence = "[",  postfix: CharSequence = "]") =
        this.joinToString(separator, prefix, postfix) {
            String.format("0x%02X", it)
        }

    @JvmOverloads
    fun String.decodeHex(): ByteArray {
        check(length % 2 == 0) { "Must have an even length" }

        return chunked(2)
            .map { it.toInt(16).toByte() }
            .toByteArray()
    }
}