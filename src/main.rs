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
    println!("Enter master pass");
    let mut master_pass = String::new();
    std::io::stdin()
        .read_line(&mut master_pass)
        .expect("Could not read a line");
    // Only need this because of ctrl+c handler
    // I wish I knew a better way to do this ¯\_(ツ)_/¯
    let keychain = Arc::new(Mutex::new(KeyChain::new(master_pass.trim()).unwrap()));
    match opt.service {
        Some(service) => {
            if opt.add {
                keychain.lock().unwrap().add_new(&service)
            }
            if let Some(pass) = keychain.lock().unwrap().get_pass(service.trim()) {
                println!("{}", pass)
            } else {
                println!("Unknown service!")
            }
        }
        None => interactive(&keychain),
    }
    keychain.lock().unwrap().dump();
}

fn interactive(keychain: &Arc<Mutex<KeyChain>>) {
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
