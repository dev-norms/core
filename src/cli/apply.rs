use clap::Args;
use log::info;

use super::RunnableCommand;

#[derive(Debug)]
#[derive(Args)]
#[clap(about = "Auto-applies fixes for any norm violations that we can.")]
pub struct Apply {}

impl RunnableCommand for Apply {
    fn run(&self) {
        info!("Fixing your shit");
    }
}
