mod cli_utils;
mod keychain_crypto;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::cli_utils::show_choice;
use self::keychain_crypto::{decrypt_file, encrypt_to_file, CryptoError};

type KeyChainResult = std::result::Result<KeyChain, CryptoError>;
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyChain {
    services: HashMap<String, String>,
    #[serde(skip)]
    master_pass: String,
}
impl KeyChain {
    pub fn new(master_pass: &str) -> KeyChainResult {
        decrypt_file("./data/services", master_pass).map_or_else(
            |err| match err {
                CryptoError::EmptyFileError => Ok(KeyChain::default(master_pass)),
                CryptoError::DecryptionError => Err(err),
            },
            |data| {
                let mut keychain = serde_json::from_str::<KeyChain>(&data).unwrap();
                keychain.master_pass = String::from(master_pass);
                return Ok(keychain);
            },
        )
    }

    pub fn from_user_input() -> KeyChainResult {
        let mut master_pass = String::new();
        println!("Enter master pass");
        std::io::stdin()
            .read_line(&mut master_pass)
            .expect("Could not read a line");
        KeyChain::new(master_pass.trim())
    }

    pub fn default(master_pass: &str) -> KeyChain {
        KeyChain {
            services: HashMap::new(),
            master_pass: String::from(master_pass),
        }
    }

    pub fn dump(&self) {
        let json = serde_json::to_string(self).expect("Could not dump the data");
        encrypt_to_file("./data/services", &self.master_pass, json).unwrap();
    }

    pub fn add_new_or_show_pass(&mut self, service: &str) {
        if let Some(user_pass) = self.get_pass(service) {
            println!("Your pass: {}", user_pass);
            if show_choice("Do you want to update the password?") {
                self.add_new(service);
            }
        } else {
            println!("Unknown service: {}", service);
            if show_choice("Do you want to add new service?") {
                self.add_new(service);
            }
        }
    }

    pub fn get_pass(&mut self, service: &str) -> Option<&String> {
        self.services.get(service)
    }

    pub fn add_new(&mut self, service: &str) {
        println!("Enter the password for service: {}", service);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read a line");
        self.services
            .insert(String::from(service), input.trim().to_string());
        println!("Password for service {} added!", service);
    }
}
