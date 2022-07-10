mod apply;
mod check;
mod server;

use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use log::trace;

#[derive(Debug)]
#[derive(Parser)]
#[clap(name = "norms")]
#[clap(version)]
#[clap(about = "Does stuff and things.")]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[enum_dispatch]
trait RunnableCommand {
    fn run(&self);
}

#[derive(Debug)]
#[derive(Subcommand)]
#[enum_dispatch(RunnableCommand)]
enum Command {
    Apply(apply::Apply),
    Check(check::Check),
    Server(server::Server),
}

pub fn run() {
    let args = Cli::parse();
    trace!("CLI args: {:#?}", args);

    Command::from(args.command).run();
}
