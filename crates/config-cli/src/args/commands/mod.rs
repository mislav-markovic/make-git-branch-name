use clap::Subcommand;

pub mod find;
pub mod make;
pub mod norm;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Make(make::MakeArgs),
    Norm(norm::NormArgs),
    Find(find::FindArgs),
}
