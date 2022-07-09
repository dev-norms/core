use clap::Args;
use log::info;

#[derive(Debug)]
#[derive(Args)]
#[clap(about = "Spins up a language server and stuff.")]
pub struct Server {}

impl Server {
    pub fn run(&self) {
        info!("Running server");
    }
}
