mod arguments;
mod git;
mod project;
mod templates;

use anyhow::Result;
use arguments::Arguments;
use clap::Parser;
use project::Project;
use std::process::exit;
use templates::Templates;

fn main() {
    if let Err(e) = main_inner() {
        eprintln!("Error - {}", e);
        exit(1);
    }
}

fn main_inner() -> Result<()> {
    let args = Arguments::parse();

    let mut project = Project::new(args);
    project.prompt_substitutions()?;

    println!("{:?}", project);

    let mut templates = Templates::new(project.project_type())?;
    templates.render()?;

    Ok(())
}
