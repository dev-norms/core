use clap::Args;
use log::info;

#[derive(Debug)]
#[derive(Args)]
#[clap(about = "Checks for norm violations.")]
pub struct Check {}

impl Check {
    pub fn run(&self) {
        info!("Looking for busted shit");
    }
}
