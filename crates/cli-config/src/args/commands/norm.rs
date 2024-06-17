use clap::Args;

#[derive(Debug, Args)]
pub struct NormArgs {
    #[arg(short, long)]
    pub version: Option<String>,
    #[arg(short, long)]
    pub r#type: Option<String>,
    #[arg(last = true, required = true)]
    pub heading: Vec<String>,
}
