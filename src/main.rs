mod arguments;
mod git;

use anyhow::{anyhow, Result};
use arguments::Arguments;
use clap::Parser;
use git::git_init;
use minijinja::{context, Environment};
use std::{
    fs::{create_dir, File},
    io::Write,
    path::Path,
    process::exit,
};

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

    // template
    let mut env = Environment::new();
    env.add_template(
        "templates/CMakeLists.txt",
        include_str!("templates/CMakeLists.txt"),
    )?;
    let tmpl = env.get_template("templates/CMakeLists.txt")?;

    // cmakelists
    let mut cmakelists = File::create(path.join("CMakeLists.txt"))?;
    cmakelists.write_all(tmpl.render(context!(name=> "exmaple-project"))?.as_bytes())?;

    git_init(path, args.git_branch.as_ref().unwrap())?;

    Ok(())
}
