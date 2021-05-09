mod argparse;
use std::string::String;

use ctrlc;
use rusty_keychain::KeyChain;
use std::sync::Arc;
use std::sync::Mutex;

use self::argparse::Opt;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    match opt.service {
        Some(service) => {
            let mut master_pass = String::new();
            println!("Enter master pass");
            std::io::stdin()
                .read_line(&mut master_pass)
                .expect("Could not read a line");
            let mut keychain = match KeyChain::new(master_pass.trim()) {
                Ok(k) => k,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
            if opt.add {
                keychain.add_new(&service)
            }
            if let Some(pass) = keychain.get_pass(service.trim()) {
                println!("{}", pass)
            } else {
                println!("Unknown service!")
            }
            keychain.dump();
        }
        None => interactive(),
    }
}

fn interactive() {
    let k = loop {
        println!("Enter master pass");
        let mut master_pass = String::new();
        std::io::stdin()
            .read_line(&mut master_pass)
            .expect("Could not read a line");
        match KeyChain::new(master_pass.trim()) {
            Ok(keychain) => break keychain,
            // TODO: proper error handling
            Err(e) => println!("{}", e),
        }
    };
    // Only need this because of ctrl+c handler
    // I wish I knew a better way to do this ¯\_(ツ)_/¯
    let keychain = Arc::new(Mutex::new(k));
    let keychain_clone = keychain.clone();

    ctrlc::set_handler(move || {
        println!("Please wait, your data is beind processed...");
        keychain_clone.lock().unwrap().dump();
        std::process::exit(128)
    })
    .unwrap();
    loop {
        println!("Enter service name or 'exit' to exit:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read a line");
        match input.trim() {
            "exit" => break,
            _ => keychain.lock().unwrap().add_new_or_show_pass(input.trim()),
        }
    }
}
