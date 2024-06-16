use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct NormArgs {
    #[arg(short, long)]
    version: Option<String>,
    #[arg(short, long)]
    r#type: Option<String>,
    #[arg(last = true, required = true)]
    heading: Vec<String>,
}
