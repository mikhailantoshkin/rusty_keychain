use std::string::String;

use ctrlc;
use rusty_keychain::KeyChain;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    println!("Enter master pass");
    let mut master_pass = String::new();
    std::io::stdin()
        .read_line(&mut master_pass)
        .expect("Could not read a line");

    let keychain = Arc::new(Mutex::new(KeyChain::new(master_pass.trim()).unwrap()));
    let k = keychain.clone();

    ctrlc::set_handler(move || {
        println!("Hello world!");
        k.lock().unwrap().dump();
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
    keychain.lock().unwrap().dump();
}
