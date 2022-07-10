use async_trait::async_trait;
use clap::Args;
use log::info;

use crate::server;

use super::RunnableCommand;

#[derive(Debug)]
#[derive(Args)]
#[clap(about = "Spins up a language server and stuff.")]
pub struct Server {}

#[async_trait]
impl RunnableCommand for Server {
    async fn run(&self) {
        info!("Running server");

        server::start();
    }
}
