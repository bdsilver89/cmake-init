use clap::Parser;
use std::path::PathBuf;

use crate::project::{CStandard, CXXStandard};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Arguments {
    // project data
    #[arg(help = "Path to generate a project into. Also used to determine project name")]
    pub path: PathBuf,

    #[arg(long, help = "Set the project name, defaults to the directory name")]
    pub name: Option<String>,

    #[arg(long, help = "Default branch name", default_value = "main")]
    pub git_branch: Option<String>,

    // project type
    #[arg(long = "exe", help = "Generate an executable")]
    pub executable: bool,

    #[arg(long = "lib", help = "Generate an static/shared library")]
    pub library: bool,

    #[arg(long = "header-lib", help = "Generate an header-only library")]
    pub header_libray: bool,

    // C features
    #[arg(long, help = "C Standard")]
    pub c_std: Option<CStandard>,

    // CXX features
    #[arg(long, help = "C Standard")]
    pub cxx_std: Option<CXXStandard>,

    // extra tools
    // FIX: should these be --enable-feat flags or --disable-feat or --no-feat flags?
    // mixing and matching these based off personal preference may be confusing
    // these should match project settings defaults
    #[arg(long, help = "Enable clang-tidy")]
    pub clang_tidy: bool,

    #[arg(long, help = "Enable cppcheck")]
    pub cppcheck: bool,
}
