use std::process::exit;

use crate::conf::Configuration;
use anyhow::{bail, Result};
use dialoguer::Confirm;

pub fn check_needed(conf: &Configuration) -> Result<()> {
    let targets = rustup_target::list()?;

    let mut to_install = vec![];
    for needed_target in conf.cargo_section.chosen_targets() {
        let Some((target, installed)) = targets.iter().find(|t| t.0 == *needed_target) else {
            bail!("")
        };

        if !installed {
            to_install.push(target.clone());
        }
    }
    if !to_install.is_empty() {
        let do_install = Confirm::new()
            .with_prompt(format!(
                "The targets {} are missing, do you want to install them?",
                to_install
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .interact()?;
        if do_install {
            rustup_target::install(&to_install)?
        } else {
            exit(1);
        }
    }
    Ok(())
}
