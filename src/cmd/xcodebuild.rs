use crate::conf::Configuration;
use anyhow::{Ok, Result};

pub fn assemble(conf: &Configuration, libs: Vec<String>) -> Result<()> {
    let mut args = vec!["-create-xcframework".to_string()];

    for lib in &libs {
        args.push("-library".into());
        args.push(lib.into());
        args.push("-headers".into());
        args.push(conf.cargo_section.include_dir.to_string());
    }

    let name = &conf.lib_name;
    let dir = &conf.build_dir;

    let out = format!("{dir}/{name}.xcframework");
    args.push("-output".into());
    args.push(out.clone());

    super::run("xcodebuild", &args, conf.cli.quiet)?;
    Ok(())
}
