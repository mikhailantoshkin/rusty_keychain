use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rusty_keychain")]
pub struct Opt {
    /// Name of the service. Don't specify to start in interactive mode
    pub service: Option<String>,
    /// Add the specified service into keychain
    #[structopt(short, long)]
    pub add: bool,
    /// Show all available services and exit
    #[structopt(short, long)]
    pub list: bool,
}
