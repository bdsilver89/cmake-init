mod arguments;
mod git;

use anyhow::{anyhow, Result};
use arguments::Arguments;
use clap::Parser;
use git::git_init;
use std::{fs::create_dir, path::Path, process::exit};

fn main() {
    let args = Arguments::parse();
    if let Err(e) = create(&args) {
        eprintln!("Error - {}", e);
        exit(1);
    }
}

fn create(args: &Arguments) -> Result<()> {
    let path = Path::new(&args.path);
    if path.exists() {
        if path.is_dir() && path.read_dir()?.next().is_some() {
            return Err(anyhow!(
                "Directory exists and is not empty: {}",
                path.display()
            ));
        }
    } else {
        create_dir(path)?;
    }

    git_init(path, args.git_branch.as_ref().unwrap())?;

    Ok(())
}
