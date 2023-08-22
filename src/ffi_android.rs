use jni::JNIEnv;
use jni::objects::{ JClass, JByteArray };
use log::{error, info, LevelFilter};
use crate::{dispatch_request};
use crate::protobuf_generated::contract::*;
use protobuf::Message;

#[no_mangle]
pub extern "system" fn Java_com_swisstronik_kotlin_SwisstronikKotlin_call<'local>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    input: JByteArray<'local>,
) -> JByteArray<'local> {
    match env.convert_byte_array(&input) {
        Ok(input) => {
            match FFIRequest::parse_from_bytes(input.as_slice()) {
                Ok(request) => {
                    match dispatch_request(request) {
                        Ok(result) => {
                            env.byte_array_from_slice(result.as_slice()).unwrap()
                        },
                        Err(e) => {
                            error!("rust: error processing request: {:?}",e);
                            env.byte_array_from_slice(e.to_string().as_bytes()).unwrap()
                        }
                    }
                },
                Err(e) => {
                    error!("rust: error parsing request: {:?}",e);
                    env.byte_array_from_slice(e.to_string().as_bytes()).unwrap()
                }
            }
        },
        Err(e) => {
            error!("rust: error decoding input array: {:?}",e);
            env.byte_array_from_slice(e.to_string().as_bytes()).unwrap()
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_swisstronik_kotlin_SwisstronikKotlin_initLogger(
    _: JNIEnv,
    _: JClass,
) {
    // Important: Logcat doesn't contain stdout / stderr so we need a custom logger.
    // An alternative solution to android_logger, is to register a callback
    // (Using the same functionality as registerCallback) to send the logs.
    // This allows to process the messages arbitrarily in the app.
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(LevelFilter::Debug)
            .with_tag("swisstronik_kotlin"),
    );
    // Log panics rather than printing them.
    // Without this, Logcat doesn't show panic message.
    log_panics::init();
    info!("init log system - done");
}
