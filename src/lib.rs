mod config;
mod keychain_crypto;

use rpassword;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::config::Config;
use self::keychain_crypto::{decrypt_file, encrypt_to_file, CryptoError};

type KeyChainResult = std::result::Result<KeyChain, CryptoError>;
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyChain {
    services: HashMap<String, String>,
    #[serde(skip)]
    master_pass: String,
    #[serde(skip)]
    cfg: Config,
}
impl KeyChain {
    pub fn new(master_pass: &str) -> KeyChainResult {
        let cfg: Config = confy::load("rusty_keychain").unwrap();

        decrypt_file(&cfg.local.path, master_pass).map_or_else(
            |err| match err {
                CryptoError::EmptyFileError => Ok(KeyChain::default(master_pass)),
                CryptoError::DecryptionError => Err(err),
            },
            |data| {
                let mut keychain = serde_json::from_str::<KeyChain>(&data).unwrap();
                // TODO: is serde somehow setting this fields? Weird
                keychain.master_pass = String::from(master_pass);
                keychain.cfg = cfg;
                return Ok(keychain);
            },
        )
    }

    pub fn from_user_input() -> KeyChainResult {
        let master_pass = rpassword::prompt_password_stdout("Password: ").unwrap();
        KeyChain::new(master_pass.trim())
    }

    pub fn default(master_pass: &str) -> KeyChain {
        KeyChain {
            services: HashMap::new(),
            master_pass: String::from(master_pass),
            cfg: confy::load("rusty_keychain").unwrap(),
        }
    }

    pub fn dump(&self) {
        let json = serde_json::to_string(self).expect("Could not dump the data");
        encrypt_to_file(&self.cfg.local.path, &self.master_pass, json).unwrap();
    }

    pub fn get_services(&mut self) -> Vec<&String> {
        self.services.keys().collect()
    }

    pub fn get_pass(&mut self, service: &str) -> Option<&String> {
        self.services.get(service)
    }

    pub fn add_new(&mut self, service: &str) {
        println!("Enter the password for service: {}", service);
        let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
        self.services
            .insert(String::from(service), pass.trim().to_string());
        println!("Password for service {} added!", service);
    }
}
