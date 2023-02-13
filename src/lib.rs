mod cmd;
mod conf;
mod ext;

use crate::conf::Configuration;
use anyhow::Result;
use cmd::{cargo, lipo, rustup, xcodebuild};
pub use conf::{XCFrameworkConfiguration, XcCli};

pub fn run(cli: XcCli) -> Result<()> {
    let conf = Configuration::load(cli)?;

    if conf.build_dir.exists() {
        fs_err::remove_dir_all(&conf.build_dir)?;
    }
    fs_err::create_dir_all(&conf.build_dir)?;

    rustup::check_needed(&conf)?;
    cargo::build(&conf)?;

    let libs = lipo::assemble_libs(&conf)?;
    xcodebuild::assemble(&conf, libs)?;
    Ok(())
}
