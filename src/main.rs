use std::string::String;

use rusty_keychain::KeyChain;

fn main() {
    println!("Enter master pass");
    let mut master_pass = String::new();
    std::io::stdin()
        .read_line(&mut master_pass)
        .expect("Could not read a line");

    let mut keychain = KeyChain::new(master_pass.trim()).unwrap();
    loop {
        println!("Enter service name or 'exit' to exit:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read a line");
        match input.trim() {
            "exit" => break,
            _ => keychain.add_new_or_show_pass(input.trim()),
        }
    }
    keychain.dump();
}
