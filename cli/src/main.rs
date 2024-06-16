mod config;

use config::Cli;

use clap::Parser;

fn main() {
    let args = config::Cli::parse();
}
