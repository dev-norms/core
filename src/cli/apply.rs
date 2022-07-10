use async_trait::async_trait;
use clap::Args;
use log::info;

use super::RunnableCommand;

#[derive(Debug)]
#[derive(Args)]
#[clap(about = "Auto-applies fixes for any norm violations that we can.")]
pub struct Apply {}

#[async_trait]
impl RunnableCommand for Apply {
    async fn run(&self) {
        info!("Fixing your shit");
    }
}
