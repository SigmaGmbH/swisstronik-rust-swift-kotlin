use std::{ffi, mem, os::raw::c_char, ptr};

#[repr(C)]
#[derive(Debug)]
pub struct ByteBuffer {
    pub ptr: *const u8,
    pub len: usize,
    pub cap: usize,
    pub err: *const c_char,
}

impl From<Vec<u8>> for ByteBuffer {
    fn from(v: Vec<u8>) -> Self {
        let ret = Self {
            ptr: v.as_ptr(),
            len: v.len(),
            cap: v.capacity(),
            err: ptr::null(),
        };
        // println!("rust: new buffer {:?}", ret);
        mem::forget(v);
        ret
    }
}

impl ByteBuffer {
    pub fn from_err<E: ToString>(e: E) -> Self {
        let err_string = ffi::CString::new(e.to_string()).unwrap();
        Self {
            ptr: ptr::null(),
            len: 0,
            cap: 0,
            err: err_string.into_raw(),
        }
    }
}

impl<E: ToString> From<Result<Vec<u8>, E>> for ByteBuffer {
    fn from(result: Result<Vec<u8>, E>) -> Self {
        match result {
            Ok(v) => Self::from(v),
            Err(e) => Self::from_err(e),
        }
    }
}