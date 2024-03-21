use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Arguments {
    #[arg(help = "Path to generate a project into. Also used to determine project name")]
    pub path: PathBuf,

    #[arg(long, help = "Set the project name, defaults to the directory name")]
    pub name: Option<String>,

    #[arg(long, help = "Default branch name", default_value = "main")]
    pub git_branch: Option<String>,

    #[arg(long = "exe", help = "Generate an executable")]
    pub executable: bool,

    #[arg(long = "lib", help = "Generate an static/shared library")]
    pub library: bool,

    #[arg(long = "header-lib", help = "Generate an header-only library")]
    pub header_libray: bool,
}
