use super::commands::Commands;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "mkgb")]
#[command(about = "Tool for managing git branhces based on issue tracking generated headings", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
