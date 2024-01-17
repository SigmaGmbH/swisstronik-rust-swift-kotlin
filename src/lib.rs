#[cfg(target_os = "ios")]
mod ffi_ios;
#[cfg(target_os = "android")]
mod ffi_android;

mod byte_buffer;
mod protobuf_generated;

extern crate protobuf;

use std::convert::TryInto;
use std::default::Default;
use protobuf::{MessageDyn};
use crate::protobuf_generated::contract::*;
use std::{any, panic, thread};
use rand::{rngs::OsRng, RngCore};
use deoxys::aead::generic_array::GenericArray;
use deoxys::aead::{Aead, KeyInit, Payload};
use deoxys::DeoxysII256;
use sha2::Sha256;
use hmac::{Hmac, Mac};

/// Size of the Deoxys-II-256-128 key in bytes.
pub const KEY_SIZE: usize = 32;
/// Size of the nonce in bytes.
pub const NONCE_SIZE: usize = 15;
/// Size of the authentication tag in bytes.
pub const TAG_SIZE: usize = 16;

pub const TX_KEY_PREFIX: &str = "IOEncryptionKeyV1";
pub const USER_KEY_PREFIX: &str = "UserEncryptionKeyV1";
type HmacSha256 = Hmac<Sha256>;

fn bytearray_to_const_size<T, const N: usize>(v: Vec<T>) -> Option<[T; N]> {
    v.try_into().ok()
}

pub fn any_err_to_string(e: Box<dyn any::Any + Send>) -> String {
    e.downcast_ref::<Box<dyn ToString>>()
        .map_or("<unknown>".into(), |e| e.to_string())
}

/// Return x25519 public key for transaction encryption
pub fn get_public_key(private_key: [u8; KEY_SIZE]) -> [u8; 32] {
    let secret = x25519_dalek::StaticSecret::from(private_key);
    let public_key = x25519_dalek::PublicKey::from(&secret);
    public_key.to_bytes()
}

/// Performes Diffie-Hellman derivation of encryption key for transaction encryption
/// * public_key – User public key
pub fn derive_shared_secret(private_key: [u8; KEY_SIZE], public_key: [u8; KEY_SIZE]) -> x25519_dalek::SharedSecret {
    let secret = x25519_dalek::StaticSecret::from(private_key);
    secret.diffie_hellman(&x25519_dalek::PublicKey::from(public_key))
}

pub fn derive_encryption_key(private_key: &[u8], salt: &[u8]) -> [u8; KEY_SIZE] {
    let mut kdf =  <HmacSha256 as KeyInit>::new_from_slice(salt).unwrap();
    kdf.update(private_key);
    let mut derived_key = [0u8; KEY_SIZE];
    let digest = kdf.finalize();
    derived_key.copy_from_slice(&digest.into_bytes()[..KEY_SIZE]);
    derived_key
}

pub fn deoxys_encrypt(private_key: &[u8; KEY_SIZE], data: &[u8]) -> Result<Vec<u8>, deoxys::Error> {
    let mut rng = OsRng;
    let mut aad = [0u8; TAG_SIZE];
    rng.fill_bytes(&mut aad);
    let mut nonce = [0u8; NONCE_SIZE];
    rng.fill_bytes(&mut nonce);
    let nonce = GenericArray::from_slice(&nonce);
    let payload = Payload {
        msg: data,
        aad: &aad,
    };
    let key = GenericArray::from_slice(private_key);
    let encrypted = DeoxysII256::new(key).encrypt(nonce, payload);
    match encrypted {
        Ok(ciphertext) => {
            let encrypted_data = [&nonce, aad.as_slice(), ciphertext.as_slice()].concat();
            Ok(encrypted_data)
        }
        Err(e) => Err(e)
    }
}

pub fn deoxys_decrypt(private_key: &[u8; KEY_SIZE], encrypted_data: &[u8]) -> Result<Vec<u8>, deoxys::Error> {
    let nonce = &encrypted_data[0..NONCE_SIZE];
    let aad = &encrypted_data[NONCE_SIZE..NONCE_SIZE+TAG_SIZE];
    let ciphertext = &encrypted_data[NONCE_SIZE+TAG_SIZE..];
    let payload = Payload {
        msg: ciphertext,
        aad: aad,
    };
    let key = GenericArray::from_slice(private_key);
    DeoxysII256::new(key).decrypt( GenericArray::from_slice(nonce), payload)
}

pub fn encrypt_ecdh(private_key: [u8; KEY_SIZE], node_public_key: [u8; KEY_SIZE], data: &[u8]) -> Result<Vec<u8>, deoxys::Error> {
    let shared_secret = derive_shared_secret(private_key, node_public_key);
    let salt = TX_KEY_PREFIX.as_bytes();
    let encryption_key = derive_encryption_key(shared_secret.as_bytes(), salt);

    let encrypted_data = deoxys_encrypt(&encryption_key, data)?;
    let public_key = get_public_key(private_key);
    let mut result = Vec::with_capacity(encrypted_data.len() + public_key.len());
    result.extend_from_slice(&public_key);
    result.extend(encrypted_data);

    Ok(result)
}

pub fn decrypt_ecdh(private_key: [u8; KEY_SIZE], node_public_key: [u8; KEY_SIZE], encrypted_data: &[u8]) -> Result<Vec<u8>, deoxys::Error> {
    let shared_secret = derive_shared_secret(private_key, node_public_key);
    let salt = TX_KEY_PREFIX.as_bytes();
    let encryption_key = derive_encryption_key(shared_secret.as_bytes(), salt);
    deoxys_decrypt(&encryption_key, encrypted_data)
}

pub fn dispatch_request(req: FFIRequest) -> Result<Vec<u8>, String> {

    println!("rust: serving sync request on {:?}", thread::current());
    let response = panic::catch_unwind(|| match req.req.expect("no req") {
        ffirequest::Req::Encrypt(r) => handle_deoxys_encrypt(r),
        ffirequest::Req::Decrypt(r) => handle_deoxys_decrypt(r),
    });

    response
        .map(|response| {
            let mut response_buf = Vec::with_capacity(response.compute_size_dyn() as usize + 1);
            response.write_to_vec_dyn(&mut response_buf).unwrap();
            response_buf
        })
        .map_err(any_err_to_string)
}


pub fn handle_deoxys_encrypt(req: DeoxysIIEncryptRequest) -> Box<dyn MessageDyn> {
    let node_public_key: Option<[u8;32]> = bytearray_to_const_size(req.node_public_key);
    if let Some(node_public_key) =  node_public_key {
        let user_private_key = derive_encryption_key(req.private_key.as_slice(),USER_KEY_PREFIX.as_bytes());
        let encrypted = encrypt_ecdh(user_private_key, node_public_key, req.data.as_slice());
        let result = match encrypted {
            Ok(ciphertext) => {
                DeoxysIIEncryptResponse {
                    response: Some(deoxys_iiencrypt_response::Response::Success(
                        DeoxysIIEncryptSuccessResponse {
                            result: ciphertext,
                            special_fields: Default::default(),
                        })),
                    special_fields: Default::default(),
                }
            }
            Err(err) => {
                let response = Some(deoxys_iiencrypt_response::Response::Failure(DeoxysIIEncryptFailureResponse {
                    special_fields: Default::default(),
                    error: Some(deoxys_iiencrypt_failure_response::Error::EncryptionError(err.to_string())),
                }));
                DeoxysIIEncryptResponse { response, special_fields: Default::default() }
            }
        };
        Box::new(result)

    } else {
        let response = Some(deoxys_iiencrypt_response::Response::Failure(DeoxysIIEncryptFailureResponse {
            special_fields: Default::default(),
            error: Some(deoxys_iiencrypt_failure_response::Error::EncryptionError("DeoxysIIEncryptError - node public key should be 32 bytes".to_string())),
        }));
        Box::new(DeoxysIIEncryptResponse { response, special_fields: Default::default() })
    }
}

pub fn handle_deoxys_decrypt(req: DeoxysIIDecryptRequest) -> Box<dyn MessageDyn> {
    let node_public_key: Option<[u8;32]> = bytearray_to_const_size(req.node_public_key);
    if let Some(node_public_key) =  node_public_key {
        let user_private_key = derive_encryption_key(req.private_key.as_slice(),USER_KEY_PREFIX.as_bytes());
        let decrypted = decrypt_ecdh(user_private_key, node_public_key, req.encrypted_data.as_slice());
        let result = match decrypted {
            Ok(plaintext) => {
                DeoxysIIDecryptResponse {
                    response: Some(deoxys_iidecrypt_response::Response::Success(
                        DeoxysIIDecryptSuccessResponse {
                            result: plaintext,
                            special_fields: Default::default(),
                        })),
                    special_fields: Default::default(),
                }
            }
            Err(err) => {
                let response = Some(deoxys_iidecrypt_response::Response::Failure(DeoxysIIDecryptFailureResponse {
                    special_fields: Default::default(),
                    error: Some(deoxys_iidecrypt_failure_response::Error::DecryptionError(err.to_string())),
                }));
                DeoxysIIDecryptResponse { response, special_fields: Default::default() }
            }
        };
        Box::new(result)

    } else {
        let response = Some(deoxys_iidecrypt_response::Response::Failure(DeoxysIIDecryptFailureResponse {
            special_fields: Default::default(),
            error: Some(deoxys_iidecrypt_failure_response::Error::DecryptionError("DeoxysIIDecryptError - node public key should be 32 bytes".to_string())),
        }));
        Box::new(DeoxysIIDecryptResponse { response, special_fields: Default::default() })
    }
}


mod tests {
    #[test]
    fn ecdh_encryption_roundrobin(){
        use crate::*;
        let mut rng = OsRng;
        let mut user_private_key = [0u8; KEY_SIZE];
        rng.fill_bytes(&mut user_private_key);
        let mut node_private_key = [0u8; KEY_SIZE];
        rng.fill_bytes(&mut node_private_key);
        let mut plaintext = [0u8;128];
        rng.fill_bytes(&mut plaintext);
        let node_public_key = get_public_key(node_private_key);
        let encrypted = encrypt_ecdh(user_private_key,node_public_key,&plaintext).unwrap();
        let data_to_decrypt = &encrypted[32..];
        let decrypted = decrypt_ecdh(user_private_key,node_public_key,data_to_decrypt);
        decrypted.unwrap();
    }
    #[test]
    fn ecdh_encryption_roundrobin_negative(){
        use crate::*;
        let mut rng = OsRng;
        let mut user_private_key = [0u8; KEY_SIZE];
        rng.fill_bytes(&mut user_private_key);
        let mut user_private_key2 = [0u8; KEY_SIZE];
        rng.fill_bytes(&mut user_private_key2);
        let mut node_private_key = [0u8; KEY_SIZE];
        rng.fill_bytes(&mut node_private_key);
        let mut plaintext = [0u8;128];
        rng.fill_bytes(&mut plaintext);
        let node_public_key = get_public_key(node_private_key);
        let encrypted = encrypt_ecdh(user_private_key,node_public_key,&plaintext).unwrap();
        let data_to_decrypt = &encrypted[32..];
        let decrypted = decrypt_ecdh(user_private_key2,node_public_key,data_to_decrypt);
        decrypted.unwrap_err();
    }

    #[test]
    fn ecdh_encryption_with_secp256k1_user_private_key(){
        use crate::*;
        let mut rng = OsRng;
        let user_secp256k1_private_key = hex::decode("C516DC17D909EFBB64A0C4A9EE1720E10D47C1BF3590A257D86EEB5FFC644D43").unwrap();

        let user_private_key = derive_encryption_key(user_secp256k1_private_key.as_slice(),USER_KEY_PREFIX.as_bytes());
        let mut plaintext = [0u8;128];
        rng.fill_bytes(&mut plaintext);
        let node_public_key = bytearray_to_const_size(hex::decode("86477673c1c6fd9d061e884f56d440b2ce03fa2fe39a2a4882357a451a7f490e").unwrap()).unwrap();
        let encrypted = encrypt_ecdh(user_private_key,node_public_key,&plaintext).unwrap();
        let data_to_decrypt = &encrypted[32..];
        let decrypted = decrypt_ecdh(user_private_key,node_public_key,data_to_decrypt);
        decrypted.unwrap();
    }
    #[test]
    fn encryption_roundrobin() {
        use crate::*;
        let mut rng = OsRng;
        let mut private_key = [0u8; KEY_SIZE];
        rng.fill_bytes(&mut private_key);
        let data = hex::decode("9f13e91a7b0b6b8b8681ac8e53edb90d1f51ed3c97a049f2658fd96c6eef3b43").unwrap();
        let encrypted = deoxys_encrypt(&private_key, data.as_slice()).unwrap();
        let decrypted = deoxys_decrypt(&private_key, encrypted.as_slice()).unwrap();
        assert_eq!(decrypted.as_slice(), data.as_slice());
    }

    #[test]
    fn encryption_roundrobin_negative() {
        use crate::*;
        let mut rng = OsRng;
        let mut private_key = [0u8; KEY_SIZE];
        rng.fill_bytes(&mut private_key);
        let mut private_key2 = [0u8; KEY_SIZE];
        rng.fill_bytes(&mut private_key2);

        let data = hex::decode("9f13e91a7b0b6b8b8681ac8e53edb90d1f51ed3c97a049f2658fd96c6eef3b43").unwrap();
        let encrypted = deoxys_encrypt(&private_key, data.as_slice()).unwrap();
        let decrypted = deoxys_decrypt(&private_key2, encrypted.as_slice());
        decrypted.unwrap_err();
    }


    #[test]
    fn should_derive_public_key_correctly() {
        use crate::*;
        let go_x25519_pk_secret = hex::decode("2fe57da347cd62431528daac5fbb290730fff684afc4cfc2ed90995f58cb3b74").unwrap();
        let private_key = hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let public_key = get_public_key(bytearray_to_const_size(private_key).unwrap());
        assert_eq!(public_key.to_vec(), go_x25519_pk_secret);
    }
    #[test]
    fn should_derive_encryption_key_correctly() {
        use crate::*;
        let go_derived_key = hex::decode("2c5235ad7a26753fb1e9c553b0912173adffef84f20b52d3e01a30a4da7b9109").unwrap();
        let user_private_key = hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let salt = TX_KEY_PREFIX.as_bytes();
        assert_eq!(derive_encryption_key(user_private_key.as_slice(),salt),go_derived_key.as_slice());
    }
}
