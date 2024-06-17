mod exec;

use cli_config::{load_cli_config, CliConfig};
use exec::dispatch_command;

fn main() {
    let args: CliConfig = load_cli_config();
    dispatch_command(&args.command);
}
