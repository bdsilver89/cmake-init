mod arguments;
mod git;
mod project;

use anyhow::{anyhow, Result};
use arguments::Arguments;
use clap::Parser;
use git::git_init;
use std::{
    fs::{create_dir, File},
    io::Write,
    path::Path,
    process::exit,
};
use tera::Tera;

fn main() {
    let args = Arguments::parse();

    let path = Path::new(&args.path);

    let name = match &args.name {
        Some(n) => n.clone(),
        None => path.file_name().unwrap().to_str().unwrap().into(),
    };

    if let Err(e) = create(&args, path, &name) {
        eprintln!("Error - {}", e);
        exit(1);
    }
}

fn create(args: &Arguments, path: &Path, name: &str) -> Result<()> {
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

    println!(
        "Creating project '{}' in directory {}",
        name,
        path.display()
    );
    if args.executable {
        println!("Generating executable");
    }

    // template
    let mut tera = Tera::default();
    tera.add_template_file(
        "templates/executable/CMakeLists.txt",
        Some("CMakeLists.txt"),
    )?;

    let mut context = tera::Context::new();
    context.insert("name", name);

    // cmakelists
    let mut cmakelists = File::create(path.join("CMakeLists.txt"))?;
    cmakelists.write_all(tera.render("CMakeLists.txt", &context)?.as_bytes())?;

    git_init(path, args.git_branch.as_ref().unwrap())?;

    Ok(())
}
