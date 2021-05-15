use ctrlc;
use rusty_keychain::KeyChain;
use shrust;
use shrust::{ExecError, Shell, ShellIO};
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

pub fn start_shell() {
    let k = loop {
        match KeyChain::from_user_input() {
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
        println!("Please wait, your data is being processed...");
        keychain_clone.lock().unwrap().dump();
        std::process::exit(128)
    })
    .unwrap();
    let mut shell = Shell::new(keychain.clone());
    shell.new_command("add", "Add a service to the keychain", 1, add_service);
    shell.new_command("show", "Show a password for a given service", 1, show_pass);
    shell.new_command_noargs("quit", "Quit", |__, keychain| {
        keychain.lock().unwrap().dump();
        Err(ExecError::Quit)
    });
    shell.new_command_noargs("list", "Quit", |io, keychain| {
        keychain
            .lock()
            .unwrap()
            .get_services()
            .into_iter()
            .for_each(|service| writeln!(io, "{}", service).unwrap());
        Ok(())
    });
    shell.run_loop(&mut ShellIO::default());
}

fn add_service(
    _: &mut shrust::ShellIO,
    keychain: &mut Arc<Mutex<KeyChain>>,
    args: &[&str],
) -> shrust::ExecResult {
    keychain.lock().unwrap().add_new(args[0]);
    Ok(())
}

fn show_pass(
    io: &mut shrust::ShellIO,
    keychain: &mut Arc<Mutex<KeyChain>>,
    args: &[&str],
) -> shrust::ExecResult {
    if let Some(pass) = keychain.lock().unwrap().get_pass(args[0]) {
        writeln!(io, "Your password: {}", pass)?;
        return Ok(());
    } else {
        writeln!(io, "Service {} is not known", args[0])?;
        return Ok(());
    }
}
