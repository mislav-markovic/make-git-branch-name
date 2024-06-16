mod config;
mod exec;

use config::Cli;
use exec::dispatch_exec;

use clap::Parser;

fn main() {
    let args: Cli = Cli::parse();
    dispatch_exec(&args.command);
}
