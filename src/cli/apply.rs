use clap::Args;
use log::info;

#[derive(Debug)]
#[derive(Args)]
#[clap(about = "Auto-applies fixes for any norm violations that we can.")]
pub struct Apply {}

impl Apply {
    pub fn run(&self) {
        info!("Fixing your shit");
    }
}
