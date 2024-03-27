mod executable;
mod header_only_library;
mod library;

use crate::project::ProjectType;
use anyhow::Result;
use std::path::PathBuf;
use tera::Tera;

#[derive(Debug)]
pub struct Templates {
    tera: Tera,
    subdirs: Vec<PathBuf>,
}

impl Templates {
    pub fn new(project_type: ProjectType) -> Result<Self> {
        let mut s = Self {
            tera: Tera::default(),
            subdirs: Vec::new(),
        };

        match project_type {
            ProjectType::Executable => {
                s.init_executable()?;
            }
            ProjectType::Library => {
                s.init_library()?;
            }
            ProjectType::HeaderOnlyLibrary => {
                s.init_header_only_library()?;
            }
        }

        Ok(s)
    }

    pub fn render(&mut self) -> Result<()> {
        // self.tera.render(template_name, context)
        Ok(())
    }

    fn init_executable(&mut self) -> Result<()> {
        // add templates
        self.tera
            .add_raw_templates(vec![(executable::CMAKE_LISTS, "CMakeLists")])
            .map_err(anyhow::Error::msg)?;

        // add subdirs
        self.subdirs.push("src".into());

        Ok(())
    }

    fn init_library(&mut self) -> Result<()> {
        self.tera
            .add_raw_templates(vec![(library::CMAKE_LISTS, "CMakeLists")])
            .map_err(anyhow::Error::msg)?;

        // add subdirs
        self.subdirs.push("include".into());
        self.subdirs.push("src".into());

        Ok(())
    }

    fn init_header_only_library(&mut self) -> Result<()> {
        self.tera
            .add_raw_templates(vec![(header_only_library::CMAKE_LISTS, "CMakeLists")])
            .map_err(anyhow::Error::msg)?;

        // add subdirs
        self.subdirs.push("include".into());

        Ok(())
    }
}
