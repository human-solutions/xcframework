use anyhow::Result;

use crate::cli::Configuration;

pub fn build(conf: &Configuration) -> Result<()> {
    let mut cmd = conf.cli.clap_cargo.build_cmd();

    if conf.target_dir != "target" {
        cmd.args(["--target-dir", conf.target_dir.as_str()]);
    }

    for target in conf.cargo_section.chosen_targets() {
        cmd.args(["--target", target]);
    }

    cmd.status()?;

    Ok(())
}
