mod commands;

use crate::args::commands::Commands;

pub fn dispatch_command(cmd: &Commands) {
    match cmd {
        Commands::Make(make_args) => commands::make::exec(&make_args),
        Commands::Norm(norm_args) => commands::norm::exec(&norm_args),
        Commands::Find(find_args) => commands::find::exec(&find_args),
    };
}
