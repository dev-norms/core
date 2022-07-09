mod apply;
mod check;
mod server;

use clap::{Parser, Subcommand};
use log::trace;

#[derive(Debug)]
#[derive(Parser)]
#[clap(name = "norms")]
#[clap(version)]
#[clap(about = "Does stuff and things.")]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug)]
#[derive(Subcommand)]
enum Commands {
    Apply(apply::Apply),
    Check(check::Check),
    Server(server::Server),
}

pub fn run() {
    let args = Cli::parse();
    trace!("CLI args: {:#?}", args);

    match args.command {
        Commands::Apply(s) => s.run(),
        Commands::Check(s) => s.run(),
        Commands::Server(s) => s.run(),
    }
}
