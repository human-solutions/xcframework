use std::io;

use crate::conf::Configuration;
use anyhow::{bail, Context, Result};
use fs_err::File;

pub fn get_module_name(conf: &Configuration) -> Result<String> {
    let mm = conf.cargo_section.include_dir.join("module.modulemap");
    let file = File::open(&mm)?;
    let content = io::read_to_string(&file)?;

    parse_module_name(&content).context(format!(
        "Failed to parse module name from modulemap file: {mm}"
    ))
}

fn parse_module_name(content: &str) -> Result<String> {
    let found_start = content.lines().find_map(|line| {
        if line.starts_with("framework module ") {
            Some(&line["framework module ".len()..])
        } else if line.starts_with("module ") {
            Some(&line["module ".len()..])
        } else {
            None
        }
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
