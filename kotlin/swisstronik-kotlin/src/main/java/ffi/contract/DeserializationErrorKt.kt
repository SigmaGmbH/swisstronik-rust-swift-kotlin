// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: protobuf_contracts/contract.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package ffi.contract;

@kotlin.jvm.JvmName("-initializedeserializationError")
public inline fun deserializationError(block: ffi.contract.DeserializationErrorKt.Dsl.() -> kotlin.Unit): ffi.contract.Contract.DeserializationError =
  ffi.contract.DeserializationErrorKt.Dsl._create(ffi.contract.Contract.DeserializationError.newBuilder()).apply { block() }._build()
/**
 * Protobuf type `ffi.contract.DeserializationError`
 */
public object DeserializationErrorKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: ffi.contract.Contract.DeserializationError.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: ffi.contract.Contract.DeserializationError.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): ffi.contract.Contract.DeserializationError = _builder.build()

    /**
     * `string field_name = 1;`
     */
    public var fieldName: kotlin.String
      @JvmName("getFieldName")
      get() = _builder.getFieldName()
      @JvmName("setFieldName")
      set(value) {
        _builder.setFieldName(value)
      }
    /**
     * `string field_name = 1;`
     */
    public fun clearFieldName() {
      _builder.clearFieldName()
    }

    /**
     * `string description = 2;`
     */
    public var description: kotlin.String
      @JvmName("getDescription")
      get() = _builder.getDescription()
      @JvmName("setDescription")
      set(value) {
        _builder.setDescription(value)
      }
    /**
     * `string description = 2;`
     */
    public fun clearDescription() {
      _builder.clearDescription()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun ffi.contract.Contract.DeserializationError.copy(block: ffi.contract.DeserializationErrorKt.Dsl.() -> kotlin.Unit): ffi.contract.Contract.DeserializationError =
  ffi.contract.DeserializationErrorKt.Dsl._create(this.toBuilder()).apply { block() }._build()
