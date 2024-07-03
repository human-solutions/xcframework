use anyhow::{bail, Context, Result};
use camino::Utf8PathBuf;
use fs_err::File;

pub fn get_module_name(mm_path: &Utf8PathBuf) -> Result<String> {
    let file = File::open(mm_path)?;
    let content = std::io::read_to_string(&file)?;

    parse_module_name(&content).context(format!(
        "Failed to parse module name from modulemap file: {mm_path}"
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
