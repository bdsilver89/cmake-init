mod arguments;
mod git;
mod project;

use anyhow::{anyhow, Result};
use arguments::Arguments;
use clap::Parser;
use git::git_init;
use project::{Project, ProjectType};
use std::{
    fs::{self, create_dir},
    path::{Path, PathBuf},
    process::exit,
};
use tera::Tera;

fn main() {
    let args = Arguments::parse();

    // TODO: collect other questions to modifiy arguments

    if let Err(e) = create(&args) {
        eprintln!("Error - {}", e);
        exit(1);
    }
}

fn create(args: &Arguments) -> Result<()> {
    let path = Path::new(&args.path);
    let name = match &args.name {
        Some(n) => n.clone(),
        None => path.file_name().unwrap().to_str().unwrap().into(),
    };

    // setup project preferences
    let mut project = Project::new(name);
    project.set_project_type(args.executable, args.library, args.header_libray)?;

    // setup output path
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

    // template setup
    let tera = Tera::default();
    let template_files = get_template_files(project.project_type())?;

    template_files
        .iter()
        .for_each(|f| println!("template file => {}", f.display()));

    // set up templating context
    let mut context = tera::Context::new();
    context.insert("project", &project);

    // generate cmakelists
    // let mut cmakelists = File::create(path.join("CMakeLists.txt"))?;
    // cmakelists.write_all(tera.render("CMakeLists.txt", &context)?.as_bytes())?;

    tera.get_template_names()
        .for_each(|t| println!("TEMPLATE = {}", t));

    git_init(path, args.git_branch.as_ref().unwrap())?;

    Ok(())
}

// fn add_executable_templates(t: &mut Tera) -> Result<()> {
//     let mut template_files = Vec::<PathBuf>::new();
//     collect_files_recursive(&mut template_files, Path::new("templates/executable"))?;
//     // let templates = fs::read_dir("templates/executable")?
//     //     .map(|res| res.map(|e| e.path()))
//     //     .collect::<Result<Vec<_>, io::Error>>()?;
//
//     template_files
//         .iter()
//         .for_each(|temp| println!("{}", temp.display()));
//
//     // FIXME: create files in output directory for each path
//     let templates: Vec<(PathBuf, Option<String>)> = template_files
//         .iter()
//         .map(|t| {
//             // let tpath = t.as_path();
//             (
//                 t.clone(),
//                 // Some(t.strip_prefix(tpath.components().first())),
//                 Some(t.file_name().unwrap().to_str().unwrap().into()),
//             )
//         })
//         .collect();
//
//     t.add_template_files(templates)?;
//
//     Ok(())
// }
//
// fn add_library_templates(_t: &mut Tera) -> Result<()> {
//     Ok(())
// }
//
// fn add_header_library_templates(_t: &mut Tera) -> Result<()> {
//     Ok(())
// }

fn get_template_files(project_type: ProjectType) -> Result<Vec<PathBuf>> {
    let mut result = Vec::<PathBuf>::new();
    match project_type {
        ProjectType::Executable => {
            collect_files_recursive(&mut result, Path::new("templates/executable"))?;
        }
        ProjectType::Library => {
            collect_files_recursive(&mut result, Path::new("templates/library"))?;
        }
        ProjectType::HeaderOnlyLibrary => {
            collect_files_recursive(&mut result, Path::new("templates/header"))?;
        }
    };

    Ok(result)
}

fn collect_files_recursive(result: &mut Vec<PathBuf>, path: &Path) -> Result<()> {
    for p in fs::read_dir(path)? {
        let full_path = p?.path();
        if full_path.is_dir() {
            collect_files_recursive(result, &full_path)?;
        } else {
            result.push(full_path);
        }
    }

    Ok(())
}
