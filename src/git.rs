use std::{ops::Index, path::Path, process::Command};

use anyhow::{anyhow, Result};
use regex::Regex;

pub fn git_init(_path: &Path, _branch: &str) -> Result<()> {
    let _git_version = git_version()?;
    // TODO: run git init command
    // println!("version = {} branch = {}", git_version, branch);
    Ok(())
}

pub fn git_version() -> Result<String> {
    let output = Command::new("git").arg("--version").output()?;
    if !output.status.success() {
        return Err(anyhow!("Could not determine git version"));
    }

    let re = Regex::new(r"\d+(\.\d+)+")?;
    let val = re.captures(std::str::from_utf8(&output.stdout)?);
    match val {
        Some(v) => Ok(v.index(0).into()),
        None => Err(anyhow!("Could not determine git version")),
    }
}
