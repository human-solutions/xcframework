mod cmd;
mod conf;
mod ext;

use crate::conf::Configuration;
use anyhow::Result;
use cmd::{cargo, lipo, targets, xcodebuild};
pub use conf::Cli;

pub fn run(cli: Cli) -> Result<()> {
    if let Some(manifest_path) = &cli.manifest_path {
        if let Some(working_dir) = manifest_path.parent() {
            println!("cd {}", working_dir);
            std::env::set_current_dir(working_dir)?;
        }
    }
    let conf = Configuration::load(cli)?;

    if conf.build_dir.exists() {
        fs_err::remove_dir_all(&conf.build_dir)?;
    }
    fs_err::create_dir_all(&conf.build_dir)?;

    targets::check_needed(&conf)?;
    cargo::build(&conf)?;
    let libs = lipo::assemble_libs(&conf)?;
    xcodebuild::assemble(&conf, libs)?;
    Ok(())
}
