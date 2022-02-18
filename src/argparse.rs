use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {
    /// Name of the service. Don't specify to start in interactive mode
    pub service: Option<String>,
    /// Add the specified service into keychain
    #[clap(short, long)]
    pub add: bool,
    /// Show all available services and exit
    #[clap(short, long)]
    pub list: bool,
}
