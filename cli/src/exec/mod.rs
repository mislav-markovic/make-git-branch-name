pub(crate) mod commands;

use crate::config::commands::Commands;

pub fn dispatch_exec(cmd: &Commands) {
    match cmd {
        Commands::Make(make_args) => println!("{:?}", make_args),
        Commands::Norm(norm_args) => println!("{:?}", norm_args),
        Commands::Find(find_args) => println!("{:?}", find_args),
    };
}
