# rusty_keychain
Simple cli password manager but a bit rusty
# Disclamer 
This project was written as a means to an end of doing something outside the comfort of The Rust Book and, hopefully, learn something along the way. Thus said, if you really want, for some odd reason, to use this software, do it at your own discretion
# How to use
## Configuration
Config file is located at `~/.config/rusty_keychain/rusty_keychain.toml` for Linux and at a standard config directory for MacOS and Windows.

Example (default) configuration
```
[local]
path='~/.rpswd'
```
### Configuration options
#### Local
`path` - path on the local machine to a storage file holding encrypted passwords

## Running

To run in interactive shell mode run 
```
cargo run
```
To execute command in CLI mode run
```
cargo run -- [FLAGS] [ARGS]
```
Example of `cargo run -- -h`
```
rusty_keychain 0.1.0

USAGE:
    rusty_keychain [FLAGS] [service]

FLAGS:
    -a, --add        Add the specified service into keychain
    -h, --help       Prints help information
    -l, --list       Show all available services and exit
    -V, --version    Prints version information

ARGS:
    <service>    Name of the service. Don't specify to start in interactive mode
```
## Interactive shell mode
Available commands when running in interactive shell mode
```
 add [SERVICE]  :  Add a service to the keychain
 help           :  Print this help
 history        :  Print commands history or run a command from it
 list           :  List all the services in the keychain
 quit           :  Quit
 show [SERVICE] :  Show a password for a given service
```
