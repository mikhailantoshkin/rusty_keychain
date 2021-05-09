use rand::Rng;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::{error, fmt};

use aes;
use ctr;
use ctr::cipher::{NewCipher, StreamCipher};
use ring::digest;
#[cfg(test)]
use tempdir::TempDir;

type Result<T> = std::result::Result<T, CryptoError>;

type Aes256Ctr = ctr::Ctr128BE<aes::Aes256>;
#[derive(Debug, Clone)]
pub enum CryptoError {
    EmptyFileError,
    DecryptionError,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CryptoError::EmptyFileError => write!(f, "File is empty"),
            CryptoError::DecryptionError => write!(f, "Wrong password"),
        }
    }
}

impl error::Error for CryptoError {}

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
        return Err(CryptoError::EmptyFileError);
    }

    let key = digest::digest(&digest::SHA256, passwd.as_bytes());
    let nonce = data.split_off(data.len() - 16);

    let mut cipher = Aes256Ctr::new(key.as_ref().into(), nonce.as_slice().into());
    cipher.apply_keystream(&mut data);

    Ok(String::from_utf8(data).map_err(|_| CryptoError::DecryptionError)?)
}

pub fn encrypt_to_file(filename: &str, passwd: &str, data: String) -> std::result::Result<(), ()> {
    let key = digest::digest(&digest::SHA256, passwd.as_bytes());
    let nonce: [u8; 16] = rand::thread_rng().gen();

    let mut cipher = Aes256Ctr::new(key.as_ref().into(), &nonce.into());
    let mut data: Vec<u8> = Vec::from(data.as_bytes());
    cipher.apply_keystream(data.as_mut_slice());
    data.extend(&nonce);
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
