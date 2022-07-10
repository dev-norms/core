use clap::Args;
use log::info;

use super::RunnableCommand;

#[derive(Debug)]
#[derive(Args)]
#[clap(about = "Spins up a language server and stuff.")]
pub struct Server {}

impl RunnableCommand for Server {
    fn run(&self) {
        info!("Running server");
    }
}
