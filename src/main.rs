use pretty_env_logger;

mod cli;

fn main() {
    pretty_env_logger::init();

    cli::run();
}
