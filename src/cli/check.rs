use async_trait::async_trait;
use clap::Args;
use log::info;

use super::RunnableCommand;

#[derive(Debug)]
#[derive(Args)]
#[clap(about = "Checks for norm violations.")]
pub struct Check {}

#[async_trait]
impl RunnableCommand for Check {
    async fn run(&self) {
        info!("Looking for busted shit");
    }
}
