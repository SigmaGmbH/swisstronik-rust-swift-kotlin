import Foundation
import SwiftProtobuf
import SwisstronikRust

struct SwisstronikRustError: Error {
    public let error: String
}

private func extractByteBuffer(_ bb: ByteBuffer) -> (Data?, SwisstronikRustError?) {
    var resError: SwisstronikRustError? = nil
    var resData: Data? = nil

    if let err = bb.err {
        resError = SwisstronikRustError(error: String(cString: err)) // copied
    } else {
        resData = Data(UnsafeRawBufferPointer(start: bb.ptr, count: Int(bb.len))) // copied
    }

    return (resData, resError)
}


func rustCall<Response: SwiftProtobuf.Message>(_ request: Ffi_Contract_FFIRequest) throws -> Response {
    let reqData = try! request.serializedData()
    let resByteBuffer = try reqData.withUnsafeBytes { buffer -> ByteBuffer in
        guard let ptr = buffer.baseAddress?.assumingMemoryBound(to: UInt8.self) else {
            throw SwisstronikRustError(error: "Swisstronik: invalid base address")
        }
        return rust_call(ptr, Int(reqData.count))
    }

    let (resData, resError) = extractByteBuffer(resByteBuffer)
    defer { rust_free(resByteBuffer) }

    if let resData = resData {
        let res = try Response(serializedData: resData)
        return res
    } else {
        throw resError!
    }
}


func SwisstronikEncrypt(privateKey: Data, nodePublicKey: Data, data: Data) throws -> Data? {
    let req = Ffi_Contract_FFIRequest.with {
        $0.encrypt = Ffi_Contract_DeoxysIIEncryptRequest.with {
            $0.privateKey = privateKey
            $0.nodePublicKey = nodePublicKey
            $0.data = data
        }
    }
    let ffiResponse: Ffi_Contract_DeoxysIIEncryptResponse = try rustCall(req)
    if let resp = ffiResponse.response {
        switch resp {
        case let .failure(failure):
            throw SwisstronikRustError(error: failure.encryptionError)
        case let .success(success):
            return success.result
        }
    } else {
        throw SwisstronikRustError(error: "Swisstronik: FFI response was nil in Encryption")
    }
}

func SwisstronikDecrypt(privateKey: Data, nodePublicKey: Data, encryptedData: Data) throws -> Data? {
    let req = Ffi_Contract_FFIRequest.with {
        $0.decrypt = Ffi_Contract_DeoxysIIDecryptRequest.with {
            $0.privateKey = privateKey
            $0.nodePublicKey = nodePublicKey
            $0.encryptedData = encryptedData
        }
    }
    let ffiResponse: Ffi_Contract_DeoxysIIDecryptResponse = try rustCall(req)
    if let resp = ffiResponse.response {
        switch resp {
        case let .failure(failure):
            throw SwisstronikRustError(error: failure.decryptionError)
        case let .success(success):
            return success.result
        }
    } else {
        throw SwisstronikRustError(error: "Swisstronik: FFI response was nil in Decryption")
    }
}

