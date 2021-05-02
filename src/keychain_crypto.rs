use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

use aes_ctr::cipher::{
    generic_array::GenericArray,
    stream::{NewStreamCipher, SyncStreamCipher},
};
use aes_ctr::Aes256Ctr;
use ring::digest;
#[cfg(test)]
use tempdir::TempDir;

type Result<T> = std::result::Result<T, EmpyFileError>;

#[derive(Debug, Clone)]
pub struct EmpyFileError;

// TODO: wrong password errors
pub fn decrypt_file(filename: &str, passwd: &str) -> Result<String> {
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)
        .expect("Could Not create or open file");
    let mut data: Vec<u8> = Vec::new();
    let data_len = BufReader::new(fd)
        .read_to_end(&mut data)
        .expect("Could not read the file");
    if data_len == 0 {
        return Err(EmpyFileError);
    }
    let key_hash = digest::digest(&digest::SHA256, passwd.as_bytes());
    let key = GenericArray::from_slice(key_hash.as_ref());
    let nonce = GenericArray::from_slice(b"and secret nonce");
    let mut cipher = Aes256Ctr::new(&key, &nonce);
    cipher.apply_keystream(&mut data);

    let ret = String::from_utf8(data).expect("Unable to convert string");
    Ok(ret)
}

pub fn encrypt_to_file(filename: &str, passwd: &str, data: String) -> std::result::Result<(), ()> {
    let key_hash = digest::digest(&digest::SHA256, passwd.as_bytes());
    let key = GenericArray::from_slice(key_hash.as_ref());
    let nonce = GenericArray::from_slice(b"and secret nonce");

    let mut cipher = Aes256Ctr::new(&key, &nonce);
    let mut data: Vec<u8> = Vec::from(data.as_bytes());
    cipher.apply_keystream(data.as_mut_slice());
    let fd = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .expect("Could Not create or open file");

    let mut writer = BufWriter::new(fd);
    writer
        .write(data.as_mut_slice())
        .expect("Could not write data");
    Ok(())
}

#[test]
fn test_crypto() {
    let dir = TempDir::new("test_dir").unwrap();
    let file_path = dir.path().join("foo");
    encrypt_to_file(file_path.to_str().unwrap(), "123", String::from("test")).unwrap();
    let data = decrypt_file(file_path.to_str().unwrap(), "123").unwrap();
    assert!(data == "test")
}
