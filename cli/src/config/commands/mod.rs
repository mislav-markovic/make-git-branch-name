use clap::Subcommand;

mod find;
mod make;
mod norm;

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Make(make::MakeArgs),
    Norm(norm::NormArgs),
    Find(find::FindArgs),
}
