mod server;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "norms")]
#[clap(bin_name = "norms")]
#[clap(version)]
#[clap(about = "Does stuff and things.")]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server(server::Server)
}

pub fn run() {
    Cli::parse();
}
