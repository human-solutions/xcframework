use std::io;

use crate::conf::Configuration;
use anyhow::{bail, Context, Result};
use camino_fs::*;
use std::fs::File;

pub fn get_module_name(conf: &Configuration) -> Result<String> {
    let mm_files = ls_modulemap_files(&conf.cargo_section.include_dir)?;
    if mm_files.len() != 1 {
        bail!(
            "Expected one modulemap file in include directory, found {count}: {mm_files:?} in {dir}",
            count = mm_files.len(),
            dir = conf.cargo_section.include_dir
        );
    }
    let mm = &mm_files[0];
    let file = File::open(mm)?;
    let content = io::read_to_string(&file)?;

    parse_module_name(&content).context(format!(
        "Failed to parse module name from modulemap file: {mm}"
    ))
}

fn parse_module_name(content: &str) -> Result<String> {
    let found_start = content.lines().find_map(|line| {
        line.strip_prefix("framework module ")
            .or_else(|| line.strip_prefix("module "))
    });

    let Some(found_start) = found_start else {
        bail!("No 'module' declaration found");
    };

    let mut module = found_start.trim_end();
    if module.ends_with('{') {
        module = module[..module.len() - 1].trim_end();
    } else {
        bail!("Expected `module <name> {{` not `{module}`");
    }
    Ok(module.to_string())
}

fn ls_modulemap_files(dir: &Utf8Path) -> Result<Vec<Utf8PathBuf>> {
    Ok(dir
        .ls()
        .files()
        .filter(|path| path.extension().map_or(false, |ext| ext == "modulemap"))
        .collect())
}
