// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: protobuf_contracts/contract.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package ffi.contract;

@kotlin.jvm.JvmName("-initializedeoxysIIDecryptSuccessResponse")
public inline fun deoxysIIDecryptSuccessResponse(block: ffi.contract.DeoxysIIDecryptSuccessResponseKt.Dsl.() -> kotlin.Unit): ffi.contract.Contract.DeoxysIIDecryptSuccessResponse =
  ffi.contract.DeoxysIIDecryptSuccessResponseKt.Dsl._create(ffi.contract.Contract.DeoxysIIDecryptSuccessResponse.newBuilder()).apply { block() }._build()
/**
 * Protobuf type `ffi.contract.DeoxysIIDecryptSuccessResponse`
 */
public object DeoxysIIDecryptSuccessResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: ffi.contract.Contract.DeoxysIIDecryptSuccessResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: ffi.contract.Contract.DeoxysIIDecryptSuccessResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): ffi.contract.Contract.DeoxysIIDecryptSuccessResponse = _builder.build()

    /**
     * `bytes result = 1;`
     */
    public var result: com.google.protobuf.ByteString
      @JvmName("getResult")
      get() = _builder.getResult()
      @JvmName("setResult")
      set(value) {
        _builder.setResult(value)
      }
    /**
     * `bytes result = 1;`
     */
    public fun clearResult() {
      _builder.clearResult()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun ffi.contract.Contract.DeoxysIIDecryptSuccessResponse.copy(block: ffi.contract.DeoxysIIDecryptSuccessResponseKt.Dsl.() -> kotlin.Unit): ffi.contract.Contract.DeoxysIIDecryptSuccessResponse =
  ffi.contract.DeoxysIIDecryptSuccessResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()
