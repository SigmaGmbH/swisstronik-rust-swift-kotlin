package com.swisstronik.kotlin

import com.google.protobuf.kotlin.toByteString

class SwisstronikException(message: String) : Exception(message)
open class SwisstronikKotlin {
    init {
        try {
            System.loadLibrary("swisstronik")
        } catch (e: UnsatisfiedLinkError) {
            throw UnsatisfiedLinkError(
                "Error linking Swisstronik Rust library. Check that the .so file " +
                        "for the current architecture is in the libs directory. Error: $e"
            )
        }
    }

    external fun initLogger()
    external fun call(input: ByteArray): ByteArray

    fun swisstronikEncrypt(
        privateKey: ByteArray,
        nodePublicKey: ByteArray,
        data: ByteArray
    ): Result<ByteArray> {
        val encryptReq = ffi.contract.Contract.DeoxysIIEncryptRequest.newBuilder()
            .setPrivateKey(privateKey.toByteString())
            .setNodePublicKey(nodePublicKey.toByteString())
            .setData(data.toByteString())
        val ffiReq = ffi.contract.Contract.FFIRequest
            .newBuilder()
            .setEncrypt(encryptReq)
            .build()
        val resp = call(ffiReq.toByteArray())

        return try {
            val decoded = ffi.contract.Contract.DeoxysIIEncryptResponse.parseFrom(resp)
            if (decoded.hasFailure()) {
                Result.failure(SwisstronikException(decoded.failure.encryptionError))
            } else {
                Result.success(decoded.success.result.toByteArray())
            }
        } catch (e: com.google.protobuf.InvalidProtocolBufferException) {
            Result.failure(SwisstronikException(String(resp, charset = Charsets.UTF_8)))
        }
    }

    fun swisstronikDecrypt(
        privateKey: ByteArray,
        nodePublicKey: ByteArray,
        encryptedData: ByteArray
    ): Result<ByteArray> {
        val decryptReq = ffi.contract.Contract.DeoxysIIDecryptRequest.newBuilder()
            .setPrivateKey(privateKey.toByteString())
            .setNodePublicKey(nodePublicKey.toByteString())
            .setEncryptedData(encryptedData.toByteString())
        val ffiReq = ffi.contract.Contract.FFIRequest
            .newBuilder()
            .setDecrypt(decryptReq)
            .build()
        val resp = call(ffiReq.toByteArray())
        return try {
            val decoded = ffi.contract.Contract.DeoxysIIDecryptResponse.parseFrom(resp)
            if (decoded.hasFailure()) {
                Result.failure(SwisstronikException(decoded.failure.decryptionError))
            } else {
                Result.success(decoded.success.result.toByteArray())
            }
        } catch (e: com.google.protobuf.InvalidProtocolBufferException) {
            Result.failure(SwisstronikException(String(resp, charset = Charsets.UTF_8)))
        }
    }

}