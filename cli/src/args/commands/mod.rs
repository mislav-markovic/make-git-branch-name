use clap::Subcommand;

pub(crate) mod find;
pub(crate) mod make;
pub(crate) mod norm;

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Make(make::MakeArgs),
    Norm(norm::NormArgs),
    Find(find::FindArgs),
}
