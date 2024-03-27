use anyhow::{anyhow, Result};
use serde::Serialize;

use crate::arguments::Arguments;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ProjectType {
    Executable,
    Library,
    HeaderOnlyLibrary,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum PackageManager {
    None,
    // Conan,
    // Vcpkg,
    // CPM,
    // FetchContent,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum TestingLibrary {
    Catch2,
    // GoogleTest,
    // DocTest,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum BenchmarkLibrary {
    None,
    // Catch2,
    // GoogleBenchmark,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum CXXStandard {
    CXX11,
    // CXX14,
    // CXX17,
    // CXX20,
    // CXX23,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum CStandard {
    // C90,
    C99,
    // C11,
    // C17,
    // C23,
}

#[derive(Debug, Serialize)]
pub struct Project {
    name: String,
    // version: Version,
    description: String,
    homepage: String,
    project_type: ProjectType,

    cxx_standard: CXXStandard,
    c_standard: CStandard,

    package_manager: PackageManager,
    testing_library: TestingLibrary,
    benchmark_library: BenchmarkLibrary,

    use_clang_tidy: bool,
    use_cppcheck: bool,

    examples: bool,
}

impl Project {
    pub fn new(args: Arguments) -> Self {
        Self {
            name: args.name.unwrap_or_default(),
            ..Default::default()
        }
    }

    // pub fn name(&self) -> &str {
    //     &self.name
    // }

    pub fn project_type(&self) -> ProjectType {
        self.project_type
    }

    pub fn prompt_substitutions(&mut self) -> Result<()> {
        self.name = inquire::Text::new("Project name")
            .with_default(self.name.as_str())
            .prompt()?;

        // TODO: version

        self.description = inquire::Text::new("Short description").prompt()?;

        // TODO: project type
        let raw_project_type = inquire::Select::new(
            "What type of project?",
            vec!["executable", "library", "header-only library"],
        )
        .prompt()?;

        match raw_project_type {
            "executable" => self.project_type = ProjectType::Executable,
            "library" => self.project_type = ProjectType::Library,
            "header-only library" => self.project_type = ProjectType::HeaderOnlyLibrary,
            _ => return Err(anyhow!("mismatched project type input")),
        };

        self.use_clang_tidy = inquire::Confirm::new("Add clang-tidy").prompt()?;
        self.use_cppcheck = inquire::Confirm::new("Add cppcheck").prompt()?;

        Ok(())
    }

    // pub fn set_project_type(&mut self, exe: bool, lib: bool, header_lib: bool) -> Result<()> {
    //     if exe && lib {
    //         return Err(anyhow!("Cannot specify executable and library"));
    //     } else if exe && header_lib {
    //         return Err(anyhow!("Cannot specify executable and header-only library"));
    //     } else if lib && header_lib {
    //         return Err(anyhow!("Cannot specify library and header-only library"));
    //     }
    //
    //     if lib {
    //         self.project_type = ProjectType::Library;
    //     } else if header_lib {
    //         self.project_type = ProjectType::HeaderOnlyLibrary;
    //     } else {
    //         self.project_type = ProjectType::Executable;
    //     }
    //     Ok(())
    // }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: "".into(),
            // version: Version::new(0, 1, 0),
            description: "".into(),
            homepage: "".into(),
            project_type: ProjectType::Executable,

            cxx_standard: CXXStandard::CXX11,
            c_standard: CStandard::C99,

            package_manager: PackageManager::None,
            testing_library: TestingLibrary::Catch2,
            benchmark_library: BenchmarkLibrary::None,

            use_clang_tidy: true,
            use_cppcheck: false,

            examples: false,
        }
    }
}
