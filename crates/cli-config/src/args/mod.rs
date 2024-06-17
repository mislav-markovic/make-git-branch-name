pub mod commands;

use clap::{Args, Parser, Subcommand, ValueEnum};
use commands::Commands;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "mkgb")]
#[command(about = "Tool for managing git branhces based on issue tracking generated headings", long_about = None)]
pub struct CliConfig {
    #[command(subcommand)]
    pub command: Commands,
}
