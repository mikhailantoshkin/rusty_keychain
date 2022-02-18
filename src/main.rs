mod argparse;
mod config;
mod shell;

use self::argparse::Opt;
use clap::Parser;
use rusty_keychain::KeyChain;

fn main() {
    let opt = Opt::parse();

    if opt.service.is_none() & !opt.list {
        return shell::start_shell();
    }

    let mut keychain = match KeyChain::from_user_input() {
        Ok(k) => k,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    if opt.list {
        return keychain
            .get_services()
            .into_iter()
            .for_each(|x| println!("{}", x));
    }
    if let Some(service) = opt.service {
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
}
