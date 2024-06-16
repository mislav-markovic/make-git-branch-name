mod args;
mod exec;

use args::Cli;
use exec::dispatch_command;

use clap::Parser;

fn main() {
    let args: Cli = Cli::parse();
    dispatch_command(&args.command);
}
