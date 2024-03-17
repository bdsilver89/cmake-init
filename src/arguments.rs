use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Arguments {
    #[arg(help = "Path to generate a project into. Also used to determine project name")]
    pub path: PathBuf,

    #[arg(long, help = "Default branch name", default_value = "main")]
    pub git_branch: Option<String>,
}
