use base64::engine::general_purpose;
use base64::Engine;
use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn md5(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(s);
    hasher.result_str()
}

pub fn md5_by_bytes(bytes: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.input(bytes);
    hasher.result_str()
}

pub fn base64_encode(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

pub fn base64_decode(base64: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(base64).unwrap()
}
