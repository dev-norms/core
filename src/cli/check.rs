use clap::Args;
use log::info;

use super::RunnableCommand;

#[derive(Debug)]
#[derive(Args)]
#[clap(about = "Checks for norm violations.")]
pub struct Check {}

impl RunnableCommand for Check {
    fn run(&self) {
        info!("Looking for busted shit");
    }
}
