use crate::conf::Configuration;
use anyhow::{Ok, Result};
use camino::Utf8PathBuf;

pub fn assemble(conf: &Configuration, frameworks: Vec<Utf8PathBuf>) -> Result<()> {
    let mut args = vec!["-create-xcframework".to_string()];

    for framework in &frameworks {
        args.push("-framework".into());
        args.push(framework.to_string());
    }

    let name = &conf.module_name()?;
    let dir = &conf.build_dir;

    let out = format!("{dir}/{name}.xcframework");
    args.push("-output".into());
    args.push(out.clone());

    super::run("xcodebuild", &args, conf.cli.quiet)?;
    Ok(())
}
