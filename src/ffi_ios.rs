use crate::byte_buffer::*;
use crate::{dispatch_request};
use crate::protobuf_generated::contract::*;
use std::slice;
use protobuf::Message;

#[no_mangle]
pub unsafe extern "C" fn rust_call(data: *const u8, len: usize) -> ByteBuffer {
    let bytes = slice::from_raw_parts(data, len);
    match FFIRequest::parse_from_bytes(bytes) {
        Ok(request) => {
            let response_buf = dispatch_request(request);
            ByteBuffer::from(response_buf)
        }
        Err(e) => {
            println!("rust: error {:?}", e);
            ByteBuffer::from_err(e)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_free(byte_buffer: ByteBuffer) {
    let ByteBuffer { ptr, len, cap, err } = byte_buffer;
    let buf = Vec::from_raw_parts(ptr as *mut u8, len, cap);
    drop(buf);
    if !err.is_null() {
        let err_string = ::std::ffi::CString::from_raw(err as *mut _);
        drop(err_string)
    }
}