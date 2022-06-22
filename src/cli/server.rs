use clap::{Args};

#[derive(Args)]
#[clap(about = "Spins up a language server and stuff.")]
pub struct Server {
    #[clap(value_parser)]
    name: Option<String>,
}
