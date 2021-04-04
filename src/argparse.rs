use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rusty_keychaion")]
pub struct Opt {
    /// Name of the service. Don't specify to start in interactive mode
    pub service: Option<String>,
    /// Add the specified service into keychain
    #[structopt(short, long)]
    pub add: bool,
}
