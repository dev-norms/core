use pretty_env_logger;

mod cli;
mod server;

fn main() {
    pretty_env_logger::init();

    cli::run();
}
