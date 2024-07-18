use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RsArgs {
    #[arg(default_value = ".")]
    /// Optional directory to list
    pub dir: PathBuf,
    #[arg(short)]
    /// Include file metadata in listing
    pub long: bool,
}
