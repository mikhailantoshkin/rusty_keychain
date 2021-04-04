mod cli_utils;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Result;

use self::cli_utils::show_choise;

#[derive(Debug)]
pub struct KeyChain {
    services: HashMap<String, String>,
    master_pass: String,
}
impl KeyChain {
    pub fn new(master_pass: &str) -> Result<KeyChain> {
        let data = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("./data/services")
            .expect("Could Not create or open file");
        let reader = BufReader::new(data);
        let mut services: HashMap<String, String> = HashMap::new();
        for line in reader.lines() {
            if let Ok(pass_line) = line {
                let v: Vec<&str> = pass_line.splitn(2, "\t").collect();
                if v.len() == 2 {
                    services.insert(String::from(v[0]), String::from(v[1]));
                }
            }
        }
        Ok(KeyChain {
            services,
            master_pass: String::from(master_pass),
        })
    }

    pub fn dump(&self) {
        let data = OpenOptions::new()
            .write(true)
            .create(true)
            .open("./data/services")
            .expect("Could Not create or open file");
        let mut writer = BufWriter::new(data);
        for (service, pass) in self.services.iter() {
            writer
                .write_fmt(format_args!("{}\t{}\n", service, pass))
                .unwrap()
        }
    }

    pub fn add_new_or_show_pass(&mut self, service: &str) {
        if let Some(user_pass) = self.get_pass(service) {
            println!("Your pass: {}", user_pass);
            if show_choise("Do you want to update the password?") {
                self.add_new(service);
            }
        } else {
            println!("Unknown service: {}", service);
            if show_choise("Do you want to add new service?") {
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
        self.services.insert(String::from(service), input);
        println!("Password for service {} added!", service);
    }
}
