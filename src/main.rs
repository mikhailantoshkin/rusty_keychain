use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::string::String;

fn show_choise(choise: &str) -> bool {
    loop {
        println!("{}? Y/n", choise);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read a line");
        match input.to_lowercase().trim() {
            "y" => return true,
            "n" => return false,
            _ => continue,
        }
    }
}

struct KeyChain {
    services: HashMap<String, String>,
    master_pass: String,
}
impl KeyChain {
    fn add_or_show_pass(&mut self, service: &str) {
        if let Some(user_pass) = self.services.get(service) {
            println!("Your pass: {}", user_pass);
        } else {
            println!("Unknown service: {}", service);
            if !show_choise("Do you want to add new service") {
                return;
            }
            println!("Enter pass");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Could not read a line");
            self.services.insert(String::from(service), input);
            println!("New service added!");
        }
    }
}

fn build_keychain(master_pass: &str) -> Result<KeyChain> {
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
            let v: Vec<&str> = pass_line.splitn(2, "\\t").collect();
            services.insert(String::from(v[0]), String::from(v[1]));
        }
    }
    Ok(KeyChain {
        services,
        master_pass: String::from(master_pass),
    })
}

fn main() {
    println!("Enter master pass");
    let mut master_pass = String::new();
    std::io::stdin()
        .read_line(&mut master_pass)
        .expect("Could not read a line");

    let mut keychain = build_keychain(master_pass.trim()).unwrap();
    for service in keychain.services.iter() {
        println!("name {}, pass {}", service.0, service.1);
    }
    println!("Master pass: {}", keychain.master_pass);

    loop {
        println!("Enter service name or 'exit' to exit:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read a line");
        match input.trim() {
            "exit" => break,
            _ => keychain.add_or_show_pass(input.trim()),
        }
    }
}
