pub mod args;

pub use args::CliConfig;

use clap::Parser;

pub fn load_cli_config() -> CliConfig {

    CliConfig::parse()
}
